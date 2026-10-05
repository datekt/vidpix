use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::Command;

pub fn encode_mp4(
    ffmpeg: &Path,
    frames_dir: &Path,
    fps: u32,
    output: &Path,
) -> Result<()> {
    let pattern = frames_dir.join("frame_%05d.png");
    let vf = "scale=1280:-2:flags=neighbor";

    let status = Command::new(ffmpeg)
        .args(["-y", "-hide_banner", "-loglevel", "error"])
        .args(["-framerate", &fps.to_string()])
        .arg("-i")
        .arg(&pattern)
        .args(["-vf", vf])
        .args(["-c:v", "libx264", "-pix_fmt", "yuv420p"])
        .arg(output)
        .status()
        .context("failed to run ffmpeg for mp4")?;

    if !status.success() {
        return Err(anyhow!("ffmpeg mp4 encoding failed"));
    }
    Ok(())
}

pub fn encode_gif(
    ffmpeg: &Path,
    frames_dir: &Path,
    fps: u32,
    output: &Path,
) -> Result<()> {
    let pattern = frames_dir.join("frame_%05d.png");
    let vf = "fps=15,scale=800:-1:flags=neighbor,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse";

    let status = Command::new(ffmpeg)
        .args(["-y", "-hide_banner", "-loglevel", "error"])
        .args(["-framerate", &fps.to_string()])
        .arg("-i")
        .arg(&pattern)
        .args(["-vf", vf, "-loop", "0"])
        .arg(output)
        .status()
        .context("failed to run ffmpeg for gif")?;

    if !status.success() {
        return Err(anyhow!("ffmpeg gif encoding failed"));
    }
    Ok(())
}