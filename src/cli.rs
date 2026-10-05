use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};

use vidpix::core::{charset::Charset, converter, encoder, ffmpeg, renderer::Renderer, scanner};

use crate::i18n::{self, Language, Lang};
use crate::ui::progress::ConverterProgress;
use crate::ui::prompts::{self, OutputFormat};
use crate::utils::paths;

pub fn run() -> Result<()> {
    let language = match prompts::choose_language()? {
        Some(0) => Language::Ru,
        Some(1) => Language::En,
        _ => {
            println!("{}", i18n::get(Language::Ru).cancelled);
            return Ok(());
        }
    };

    let lang = i18n::get(language);
    let bin_dir = paths::binary_dir()?;

    let videos = scanner::find_videos(&bin_dir)?;
    if videos.is_empty() {
        println!("{}", lang.no_videos);
        return Ok(());
    }

    let names: Vec<String> = videos
        .iter()
        .map(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("?")
                .to_string()
        })
        .collect();

    let video_idx = match prompts::choose_video(&lang, &names)? {
        Some(i) => i,
        None => {
            println!("{}", lang.cancelled);
            return Ok(());
        }
    };
    let video_path = &videos[video_idx];

    let charset: Charset = match prompts::choose_charset(&lang)? {
        Some(c) => c,
        None => {
            println!("{}", lang.cancelled);
            return Ok(());
        }
    };

    let format = match prompts::choose_format(&lang)? {
        Some(f) => f,
        None => {
            println!("{}", lang.cancelled);
            return Ok(());
        }
    };

    convert(&lang, &bin_dir, video_path, charset, format)
}

fn convert(
    lang: &Lang,
    bin_dir: &Path,
    video_path: &Path,
    charset: Charset,
    format: OutputFormat,
) -> Result<()> {
    let fps: u32 = 12;
    let out_width: u32 = 80;

    println!("{}", lang.probing);
    let ffmpeg_path = ensure_ffmpeg(lang, bin_dir)?;
    let info = ffmpeg::probe(&ffmpeg_path, video_path)?;

    println!("{}", lang.converting);
    let extract_progress = ConverterProgress::new(&lang.converting);
    let (out_height, frames_raw) = ffmpeg::extract_frames(
        &ffmpeg_path,
        video_path,
        &info,
        out_width,
        fps,
        |done, total| extract_progress.update(done, total),
    )?;
    extract_progress.finish();

    let renderer = Renderer::new(out_width, out_height)?;
    let tmp = tempfile::tempdir().context("failed to create temp dir")?;
    let frames_dir = tmp.path();

    println!("{}", lang.rendering_frames);
    let render_progress = ConverterProgress::new(&lang.rendering_frames);
    let total = frames_raw.len() as u32;

    for (i, frame) in frames_raw.iter().enumerate() {
        let ascii = converter::frame_to_ascii(frame, out_width, out_height, charset);
        let img = renderer.render(&ascii);
        let name = format!("frame_{:05}.png", i + 1);
        img.save(frames_dir.join(&name))
            .with_context(|| format!("failed to save frame {}", i + 1))?;
        if (i + 1) as u32 % 5 == 0 || (i + 1) as u32 == total {
            render_progress.update((i + 1) as u32, total);
        }
    }
    render_progress.finish();

    println!("{}", lang.encoding_video);
    let ext = match format {
        OutputFormat::Mp4 => "mp4",
        OutputFormat::Gif => "gif",
    };
    let out_path = paths::output_path(video_path, ext);

    match format {
        OutputFormat::Mp4 => encoder::encode_mp4(&ffmpeg_path, frames_dir, fps, &out_path)?,
        OutputFormat::Gif => encoder::encode_gif(&ffmpeg_path, frames_dir, fps, &out_path)?,
    }

    println!("{}", lang.done);
    println!("{}", lang.output_saved.replace("{}", &out_path.display().to_string()));

    Ok(())
}

fn ensure_ffmpeg(lang: &Lang, bin_dir: &Path) -> Result<PathBuf> {
    let name = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };
    let local = bin_dir.join(name);
    if local.exists() {
        return Ok(local);
    }
    println!("{}", lang.downloading_ffmpeg);
    let path = ffmpeg::ensure_ffmpeg(bin_dir)
        .map_err(|e| anyhow!("{}: {}", lang.error_prefix, e))?;
    Ok(path)
}