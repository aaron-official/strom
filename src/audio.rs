use crate::models::{ChapterMeta, FfprobeOutput};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

pub async fn get_chapters(file_path: &Path) -> Result<Vec<ChapterMeta>> {
    let output = Command::new("ffprobe")
        .args(["-v", "quiet", "-print_format", "json", "-show_chapters"])
        .arg(file_path)
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

pub async fn convert_single(input: &Path, output: &Path) -> Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-y") // Overwrite
        .arg("-i")
        .arg(input)
        .arg("-vn") // No video
        .args(["-c:a", "libmp3lame", "-q:a", "2"])
        .arg(output)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("FFmpeg conversion failed"))
    }
}

pub async fn convert_split_chapter(
    input: &Path,
    output: &Path,
    start_time: &str,
    end_time: &str,
) -> Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input)
        .arg("-vn")
        .arg("-ss")
        .arg(start_time)
        .arg("-to")
        .arg(end_time)
        .args(["-c:a", "libmp3lame", "-q:a", "2"])
        .arg(output)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("FFmpeg chapter extraction failed"))
    }
}