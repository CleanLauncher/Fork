use crate::models::DownloadTask;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("CleanLauncher-DownloadManager/1.0")
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn download(&self, task: &DownloadTask) -> Result<(), String> {
        let response = self
            .client
            .get(&task.url)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}", response.status()));
        }

        let path = Path::new(&task.destination);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let mut file = File::create(&task.destination).map_err(|e| format!("Failed to create file: {}", e))?;

        let bytes = response.bytes().await.map_err(|e| format!("Failed to read bytes: {}", e))?;
        file.write_all(&bytes).map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }
}
