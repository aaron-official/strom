use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub enum ConversionStatus {
    Ready,
    ExtractingMetadata,
    Converting(f64), // Progress percentage 0.0 to 1.0
    Done,
    Error(String),
}

// Ensure AudioFile has a way to track total duration if needed for progress calcs
#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: std::path::PathBuf,
    pub filename: String,
    pub selected: bool,
    pub status: ConversionStatus,
    pub duration_ms: u64, // Added to store total length for progress calculation
}

#[derive(Debug, Deserialize)]
pub struct FfprobeOutput {
    pub chapters: Vec<ChapterMeta>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChapterMeta {
    pub id: i64,
    pub start_time: String,
    pub end_time: String,
    pub tags: ChapterTags,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChapterTags {
    pub title: String,
}

pub enum ConversionMode {
    Single,
    Split,
}
