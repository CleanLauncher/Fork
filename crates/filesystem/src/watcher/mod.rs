use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use error::Result;

#[derive(Debug, Clone)]
pub struct FileSnapshot {
    pub path: PathBuf,
    pub modified: SystemTime,
    pub size: u64,
}

pub struct FileWatcher {
    watched_dirs: HashMap<PathBuf, Vec<FileSnapshot>>,
}

impl FileWatcher {
    pub fn new() -> Self {
        FileWatcher {
            watched_dirs: HashMap::new(),
        }
    }

    pub fn watch_dir(&mut self, dir: &str) -> Result<()> {
        let path = PathBuf::from(dir);
        let snapshots = self.snapshot_dir(&path)?;
        self.watched_dirs.insert(path, snapshots);
        Ok(())
    }

    pub fn unwatch_dir(&mut self, dir: &str) {
        self.watched_dirs.remove(&PathBuf::from(dir));
    }

    pub fn check_changes(&mut self, dir: &str) -> Result<Vec<String>> {
        let path = PathBuf::from(dir);
        let current = self.snapshot_dir(&path)?;
        let previous = self.watched_dirs.get(&path).cloned().unwrap_or_default();
        let mut changes = Vec::new();

        for snap in &current {
            if !previous.iter().any(|p| p.path == snap.path) {
                changes.push(format!("+ {}", snap.path.display()));
            }
        }

        for snap in &previous {
            if !current.iter().any(|p| p.path == snap.path) {
                changes.push(format!("- {}", snap.path.display()));
            }
        }

        self.watched_dirs.insert(path, current).ok_or(()).ok();

        Ok(changes)
    }

    fn snapshot_dir(&self, dir: &Path) -> Result<Vec<FileSnapshot>> {
        let mut snapshots = Vec::new();
        if !dir.exists() {
            return Ok(snapshots);
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let metadata = fs::metadata(&path)?;
                snapshots.push(FileSnapshot {
                    path,
                    modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                    size: metadata.len(),
                });
            }
        }
        Ok(snapshots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watcher_empty() {
        let mut watcher = FileWatcher::new();
        let dir = std::env::temp_dir().join("watcher_test");
        fs::create_dir_all(&dir).ok();
        watcher.watch_dir(dir.to_str().unwrap()).unwrap();
        let changes = watcher.check_changes(dir.to_str().unwrap()).unwrap();
        assert!(changes.is_empty());
        fs::remove_dir_all(&dir).ok();
    }
}
