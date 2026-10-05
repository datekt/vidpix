use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const VIDEO_EXTS: &[&str] = &["mp4", "mov", "avi", "mkv", "webm", "m4v", "flv", "wmv"];

pub fn find_videos(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    for entry in WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_ascii_lowercase();
            if VIDEO_EXTS.contains(&ext.as_str()) {
                found.push(path.to_path_buf());
            }
        }
    }
    found.sort();
    Ok(found)
}