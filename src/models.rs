use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: PathBuf,
    pub filename: String,
    pub selected: bool,
    pub status: ConversionStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConversionStatus {
    Ready,
    ExtractingMetadata,
    Converting,
    Done,
    Error(String),
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
