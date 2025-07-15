#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(i64);

/// need [`Clone`] and [`PartialEq`] to be used in sycamore [`Keyed`] lists
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct FileData {
    pub id: u32,
    pub path: String,
    pub creation_time: Timestamp,
}
