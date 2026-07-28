use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadTask {
    pub url: String,
    pub destination: String,
    pub expected_sha256: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Pending,
    Downloading { bytes_downloaded: u64, total_bytes: Option<u64> },
    Hashing,
    Completed,
    Failed(String),
}
