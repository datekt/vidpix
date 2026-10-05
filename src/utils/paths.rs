use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};

pub fn binary_dir() -> Result<PathBuf> {
    let exe = env::current_exe().context("failed to get current exe path")?;
    let dir = exe
        .parent()
        .context("failed to get parent of exe")?
        .to_path_buf();
    Ok(dir)
}

pub fn output_path(video: &Path, ext: &str) -> PathBuf {
    let stem = video
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let dir = video.parent().unwrap_or_else(|| Path::new("."));
    dir.join(format!("{}_vidpix.{}", stem, ext))
}