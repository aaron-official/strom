use crate::models::{ChapterMeta, FfprobeOutput};
use anyhow::{Context, Result};
use std::path::Path;
use tokio::process::Command;

pub async fn get_chapters(file_path: &Path) -> Result<Vec<ChapterMeta>> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "quiet",
            "-print_format", "json",
            "-show_chapters",
            file_path.to_str().unwrap(),
        ])
        .output()
        .await
        .context("Failed to execute ffprobe")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("ffprobe failed"));
    }

    let ffprobe_out: FfprobeOutput = serde_json::from_slice(&output.stdout)
        .context("Failed to parse ffprobe json")?;

    Ok(ffprobe_out.chapters)
}
