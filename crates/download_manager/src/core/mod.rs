use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use error::{CoreError, Result};

use crate::models::DownloadTask;

pub struct Downloader;

impl Downloader {
    pub fn new() -> Self {
        Downloader
    }

    pub fn download(&self, task: &DownloadTask) -> Result<()> {
        let path = Path::new(&task.destination);

        let existing_bytes = if path.exists() {
            fs::metadata(path).map(|m| m.len()).unwrap_or(0)
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            0
        };

        let result = http_client::download_to_file_with_resume(
            &task.url,
            &task.destination,
            existing_bytes,
            None,
        )?;

        if result.status != 200 && result.status != 206 {
            return Err(CoreError::Http(format!(
                "Download failed: HTTP {}",
                result.status
            )));
        }

        if let Some(ref expected_hash) = task.expected_sha256 {
            let file_bytes = fs::read(&task.destination)?;
            let computed = hashing::sha256(&file_bytes);
            if !computed.eq_ignore_ascii_case(expected_hash) {
                fs::remove_file(&task.destination)?;
                return Err(CoreError::InvalidData(format!(
                    "Hash mismatch for {}: expected {}, got {}",
                    task.url, expected_hash, computed
                )));
            }
        }

        Ok(())
    }

    pub fn download_batch(&self, tasks: &[DownloadTask], max_concurrent: usize) -> Vec<Result<()>> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let counter = AtomicUsize::new(0);

        let mut handles = Vec::new();
        for task in tasks {
            let task = task.clone();
            let results = Arc::clone(&results);
            let handle = std::thread::spawn(move || {
                let result = Downloader::new().download(&task);
                let mut results = results.lock().unwrap();
                results.push(result);
                counter.fetch_add(1, Ordering::SeqCst);
            });
            handles.push(handle);

            if handles.len() >= max_concurrent {
                if let Some(handle) = handles.drain(..1).next() {
                    handle.join().unwrap();
                }
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }

    pub fn download_parallel(&self, tasks: &[DownloadTask]) -> Vec<Result<()>> {
        self.download_batch(tasks, 4)
    }

    pub fn verify_download(path: &str, expected_sha256: &str) -> Result<bool> {
        let bytes = fs::read(path)?;
        let computed = hashing::sha256(&bytes);
        Ok(computed.eq_ignore_ascii_case(expected_sha256))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DownloadTask;

    #[test]
    fn test_batch_download_empty() {
        let downloader = Downloader::new();
        let results = downloader.download_batch(&[], 4);
        assert!(results.is_empty());
    }

    #[test]
    fn test_verify_hash_failure() {
        let result = Downloader::verify_download("/nonexistent/file", "abc");
        assert!(result.is_err());
    }
}
