/// need [`Clone`] and [`PartialEq`] to be used in sycamore [`Keyed`] lists
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct FileData {
    pub id: u32,
    pub path: String,
}
