pub mod core;
pub mod models;
pub mod utils;

pub use core::DownloadManager;
pub use models::{DownloadStatus, DownloadTask};
pub use utils::verify_file_hash;

use core::Downloader;
use models::DownloadTask;

pub async fn download_file(task: &DownloadTask) -> Result<(), String> {
    log::info!("Starting download for {}", task.url);
    let downloader = Downloader::new();

    // Perform download
    downloader.download(task).await?;

    // Verify hash if provided
    if let Some(expected_hash) = &task.expected_sha256 {
        log::info!("Verifying SHA256 hash for {}", task.destination);
        match hash_util::verify_file_hash(&task.destination, expected_hash) {
            Ok(true) => log::info!("Hash verified successfully!"),
            Ok(false) => return Err("Hash verification failed: mismatch".into()),
            Err(e) => return Err(format!("Hash verification failed to read file: {}", e)),
        }
    }

    Ok(())
}
