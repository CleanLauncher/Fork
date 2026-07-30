use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use error::{CoreError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationManifest {
    pub current_version: u32,
    pub migrations: Vec<MigrationEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationEntry {
    pub from_version: u32,
    pub to_version: u32,
    pub description: String,
    pub date: String,
}

pub trait DataMigration: Send + Sync {
    fn version(&self) -> u32;
    fn description(&self) -> &str;
    fn migrate(&self, data: &mut HashMap<String, String>) -> Result<()>;
    fn migrate_file(&self, path: &Path) -> Result<()> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let mut data = crate::core::parse_ini(&content);
            self.migrate(&mut data)?;
            let serialized = crate::core::serialize_ini(&data);
            fs::write(path, &serialized)?;
        }
        Ok(())
    }
}

pub struct MigrationManager {
    state_path: PathBuf,
    migrations: Vec<Box<dyn DataMigration>>,
}

impl MigrationManager {
    pub fn new(state_dir: &str) -> Self {
        let path = PathBuf::from(state_dir).join("migration_state.json");
        fs::create_dir_all(Path::new(state_dir)).ok();
        MigrationManager {
            state_path: path,
            migrations: Vec::new(),
        }
    }

    pub fn register(&mut self, migration: Box<dyn DataMigration>) {
        self.migrations.push(migration);
    }

    pub fn pending_migrations(&self) -> Result<Vec<&dyn DataMigration>> {
        let current_version = self.current_version()?;
        let mut pending: Vec<&dyn DataMigration> = self
            .migrations
            .iter()
            .filter(|m| m.version() > current_version)
            .map(|m| m.as_ref())
            .collect();
        pending.sort_by(|a, b| a.version().cmp(&b.version()));
        Ok(pending)
    }

    pub fn run_all(&mut self) -> Result<Vec<MigrationResult>> {
        let mut results = Vec::new();
        let pending = self.pending_migrations()?;

        for migration in &pending {
            match self.run_migration(migration) {
                Ok(()) => {
                    results.push(MigrationResult {
                        version: migration.version(),
                        description: migration.description().to_string(),
                        success: true,
                        error: None,
                    });
                }
                Err(e) => {
                    results.push(MigrationResult {
                        version: migration.version(),
                        description: migration.description().to_string(),
                        success: false,
                        error: Some(e.to_string()),
                    });
                    return Ok(results);
                }
            }
        }

        Ok(results)
    }

    pub fn run_to_version(&mut self, target_version: u32) -> Result<Vec<MigrationResult>> {
        let mut results = Vec::new();
        let current = self.current_version()?;

        let to_run: Vec<&dyn DataMigration> = self
            .migrations
            .iter()
            .filter(|m| m.version() > current && m.version() <= target_version)
            .map(|m| m.as_ref())
            .collect();

        for migration in &to_run {
            match self.run_migration(migration) {
                Ok(()) => {
                    results.push(MigrationResult {
                        version: migration.version(),
                        description: migration.description().to_string(),
                        success: true,
                        error: None,
                    });
                }
                Err(e) => {
                    results.push(MigrationResult {
                        version: migration.version(),
                        description: migration.description().to_string(),
                        success: false,
                        error: Some(e.to_string()),
                    });
                    return Ok(results);
                }
            }
        }

        Ok(results)
    }

    pub fn current_version(&self) -> Result<u32> {
        if !self.state_path.exists() {
            return Ok(0);
        }
        let content = fs::read_to_string(&self.state_path)?;
        let manifest: MigrationManifest =
            serde_json::from_str(&content).map_err(CoreError::Json)?;
        Ok(manifest.current_version)
    }

    pub fn needs_migration(&self) -> Result<bool> {
        Ok(!self.pending_migrations()?.is_empty())
    }

    fn update_state(&self, version: u32) -> Result<()> {
        let manifest = MigrationManifest {
            current_version: version,
            migrations: Vec::new(),
        };
        let json = serde_json::to_string_pretty(&manifest).map_err(CoreError::Json)?;
        fs::write(&self.state_path, &json)?;
        Ok(())
    }

    fn run_migration(&self, migration: &dyn DataMigration) -> Result<()> {
        migration.migrate(&mut HashMap::new())?;
        self.update_state(migration.version())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MigrationResult {
    pub version: u32,
    pub description: String,
    pub success: bool,
    pub error: Option<String>,
}

// Built-in migrations
pub struct MigrationV1toV2;

impl DataMigration for MigrationV1toV2 {
    fn version(&self) -> u32 {
        2
    }

    fn description(&self) -> &str {
        "Migrate from v1 to v2: Rename 'instance_dir' to 'instances_folder', add 'theme' setting"
    }

    fn migrate(&self, data: &mut HashMap<String, String>) -> Result<()> {
        if let Some(val) = data.remove("General/instance_dir") {
            data.insert("General/instances_folder".to_string(), val);
        }
        data.entry("General/theme".to_string())
            .or_insert_with(|| "system".to_string());
        data.entry("General/locale".to_string())
            .or_insert_with(|| "en-US".to_string());
        Ok(())
    }
}

pub struct MigrationV2toV3;

impl DataMigration for MigrationV2toV3 {
    fn version(&self) -> u32 {
        3
    }

    fn description(&self) -> &str {
        "Migrate from v2 to v3: Add Java settings, memory defaults"
    }

    fn migrate(&self, data: &mut HashMap<String, String>) -> Result<()> {
        data.entry("Java/max_memory".to_string())
            .or_insert_with(|| "2048".to_string());
        data.entry("Java/min_memory".to_string())
            .or_insert_with(|| "512".to_string());
        data.entry("Java/java_path".to_string())
            .or_insert_with(|| "java".to_string());
        data.entry("Java/jvm_args".to_string())
            .or_insert_with(|| "-XX:+UseG1GC".to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_to_v2_migration() {
        let mut data = HashMap::new();
        data.insert(
            "General/instance_dir".to_string(),
            "/tmp/instances".to_string(),
        );

        let migration = MigrationV1toV2;
        migration.migrate(&mut data).unwrap();

        assert!(!data.contains_key("General/instance_dir"));
        assert_eq!(
            data.get("General/instances_folder").unwrap(),
            "/tmp/instances"
        );
        assert_eq!(data.get("General/theme").unwrap(), "system");
    }

    #[test]
    fn test_v2_to_v3_migration() {
        let mut data = HashMap::new();
        let migration = MigrationV2toV3;
        migration.migrate(&mut data).unwrap();

        assert_eq!(data.get("Java/max_memory").unwrap(), "2048");
        assert_eq!(data.get("Java/min_memory").unwrap(), "512");
    }

    #[test]
    fn test_migration_manager() {
        let dir = std::env::temp_dir().join("migration_test");
        fs::create_dir_all(&dir).ok();

        let mut manager = MigrationManager::new(dir.to_str().unwrap());
        manager.register(Box::new(MigrationV1toV2));
        manager.register(Box::new(MigrationV2toV3));

        assert!(manager.needs_migration().unwrap());

        let results = manager.run_all().unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));

        assert_eq!(manager.current_version().unwrap(), 3);
        assert!(!manager.needs_migration().unwrap());

        fs::remove_dir_all(&dir).ok();
    }
}
