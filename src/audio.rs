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

pub async fn get_duration_ms(file_path: &Path) -> Result<u64> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(file_path)
        .output()
        .await?;

    let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let duration_secs: f64 = duration_str.parse().context("Failed to parse duration")?;
    Ok((duration_secs * 1000.0) as u64)
}

pub fn get_output_path(input: &Path, is_split: bool) -> Result<std::path::PathBuf> {
    let parent = input.parent().context("No parent dir")?;
    let converted_dir = parent.join("converted");
    if !converted_dir.exists() {
        std::fs::create_dir_all(&converted_dir)?;
    }

    if is_split {
        let stem = input.file_stem().context("No file stem")?.to_string_lossy();
        let split_dir = converted_dir.join(format!("{}_chapters", stem));
        if !split_dir.exists() {
            std::fs::create_dir_all(&split_dir)?;
        }
        Ok(split_dir)
    } else {
        Ok(converted_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_get_duration_ms() {
        let path = PathBuf::from("01 The Way of Kings.m4b");
        // This should fail initially because the function is not implemented
        let duration = get_duration_ms(&path).await.unwrap();
        assert!(duration > 0);
    }

    #[test]
    fn test_get_output_path() {
        let path = PathBuf::from("test_dir/test.m4b");
        // Mocking parent directory behavior for test_dir/test.m4b
        // We'll use a real temp dir if needed, but for now just test the logic
        let output = get_output_path(&path, false).unwrap();
        assert!(output.ends_with("converted"));
        
        let output_split = get_output_path(&path, true).unwrap();
        assert!(output_split.ends_with("converted/test_chapters"));
    }
}