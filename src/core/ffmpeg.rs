use anyhow::{anyhow, Context, Result};
use ffmpeg_sidecar::download::auto_download;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub duration_secs: f64,
}

pub fn ensure_ffmpeg(dir: &Path) -> Result<PathBuf> {
    let name = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };
    let local = dir.join(name);
    if local.exists() {
        return Ok(local);
    }

    auto_download().context("failed to download ffmpeg")?;

    let downloaded = ffmpeg_sidecar::paths::ffmpeg_path();
    if downloaded.exists() {
        std::fs::copy(&downloaded, &local).ok();
        if local.exists() {
            return Ok(local);
        }
        return Ok(downloaded);
    }

    Err(anyhow!("ffmpeg not found after download"))
}

pub fn probe(ffmpeg: &Path, video: &Path) -> Result<VideoInfo> {
    let output = Command::new(ffmpeg)
        .args(["-hide_banner", "-i"])
        .arg(video)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("failed to run ffmpeg probe")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    parse_probe(&stderr)
}

fn parse_probe(stderr: &str) -> Result<VideoInfo> {
    let mut width = 0u32;
    let mut height = 0u32;
    let mut duration = 0.0f64;

    for line in stderr.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("Duration:") && duration == 0.0 {
            let rest = trimmed.trim_start_matches("Duration:").trim();
            let t = rest.trim_end_matches(',');
            let parts: Vec<&str> = t.split(':').collect();
            if parts.len() == 3 {
                let h: f64 = parts[0].trim().parse().unwrap_or(0.0);
                let m: f64 = parts[1].trim().parse().unwrap_or(0.0);
                let s: f64 = parts[2].trim().parse().unwrap_or(0.0);
                duration = h * 3600.0 + m * 60.0 + s;
            }
        }

        if trimmed.contains("Video:") && width == 0 {
            if let Some((w, h)) = parse_resolution(trimmed) {
                width = w;
                height = h;
            }
        }
    }

    if width == 0 || height == 0 {
        return Err(anyhow!("could not determine video resolution"));
    }

    Ok(VideoInfo {
        width,
        height,
        duration_secs: duration,
    })
}

fn parse_resolution(line: &str) -> Option<(u32, u32)> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'x' {
            let mut start = i;
            while start > 0 && bytes[start - 1].is_ascii_digit() {
                start -= 1;
            }
            let mut end = i + 1;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if start < i && end > i + 1 {
                let w_str = std::str::from_utf8(&bytes[start..i]).ok()?;
                let h_str = std::str::from_utf8(&bytes[i + 1..end]).ok()?;
                let w: u32 = w_str.parse().ok()?;
                let h: u32 = h_str.parse().ok()?;
                if w > 0 && h > 0 {
                    return Some((w, h));
                }
            }
        }
        i += 1;
    }
    None
}

pub fn extract_frames(
    ffmpeg: &Path,
    video: &Path,
    info: &VideoInfo,
    out_width: u32,
    fps: u32,
    mut on_progress: impl FnMut(u32, u32),
) -> Result<(u32, Vec<Vec<u8>>)> {
    let ratio = info.height as f64 / info.width as f64;
    let out_height = ((out_width as f64 * ratio) / 2.0).round() as u32;
    let out_height = out_height.max(1);

    let vf = format!(
        "fps={},scale={}:{},format=gray",
        fps, out_width, out_height
    );

    let mut child = Command::new(ffmpeg)
        .args(["-hide_banner", "-loglevel", "error", "-i"])
        .arg(video)
        .args(["-vf", &vf, "-f", "rawvideo", "-pix_fmt", "gray", "pipe:1"])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("failed to spawn ffmpeg")?;

    let mut stdout = child.stdout.take().context("no stdout from ffmpeg")?;
    let frame_size = (out_width * out_height) as usize;
    let mut frames: Vec<Vec<u8>> = Vec::new();
    let mut buf = vec![0u8; frame_size];

    let estimated_total = if info.duration_secs > 0.0 {
        (info.duration_secs * fps as f64).ceil() as u32
    } else {
        0
    };

    loop {
        match stdout.read_exact(&mut buf) {
            Ok(()) => {
                frames.push(buf.clone());
                if estimated_total > 0 && frames.len() % 5 == 0 {
                    on_progress(frames.len() as u32, estimated_total);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        }
    }

    let _ = child.wait();

    if frames.is_empty() {
        return Err(anyhow!("no frames extracted"));
    }

    if estimated_total > 0 {
        on_progress(frames.len() as u32, estimated_total);
    }

    Ok((out_height, frames))
}