use std::path::PathBuf;

use chrono::{DateTime, Utc};

/// in milliseconds
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    pub fn format(&self) -> String {
        DateTime::<Utc>::from_timestamp_millis(self.0)
            .unwrap()
            .to_rfc3339()
    }
}

/// need [`Clone`] and [`PartialEq`] to be used in sycamore [`Keyed`] lists
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct FileData {
    pub id: u32,
    pub file_name: String,
    pub creation_time: Timestamp,
    pub extension: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone, PartialEq, Eq)]
pub struct PickedDirectory {
    pub directory: PathBuf,
    pub files: Vec<FileData>,
}
