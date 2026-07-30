use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use error::{CoreError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub game_version: String,
    pub loader: LoaderConfig,
    pub memory: MemoryConfig,
    pub resolution: ResolutionConfig,
    pub java_args: Vec<String>,
    pub game_args: Vec<String>,
    pub environment: HashMap<String, String>,
    pub notes: String,
    pub icon_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoaderConfig {
    pub loader_type: String,
    pub loader_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub min_mb: u32,
    pub max_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceProfile {
    pub config: InstanceConfig,
    pub path: PathBuf,
    pub last_played: u64,
    pub total_play_time: u64,
    pub mod_count: u32,
    pub modpack_id: Option<String>,
    pub modpack_version: Option<String>,
    pub modpack_platform: Option<String>,
}

impl Default for InstanceConfig {
    fn default() -> Self {
        InstanceConfig {
            name: String::new(),
            game_version: "latest_release".into(),
            loader: LoaderConfig {
                loader_type: "vanilla".into(),
                loader_version: String::new(),
            },
            memory: MemoryConfig {
                min_mb: 512,
                max_mb: 2048,
            },
            resolution: ResolutionConfig {
                width: 854,
                height: 480,
                fullscreen: false,
            },
            java_args: vec![
                "-XX:+UseG1GC".into(),
                "-XX:+UnlockExperimentalVMOptions".into(),
                "-XX:G1NewSizePercent=20".into(),
                "-XX:G1ReservePercent=20".into(),
                "-XX:MaxGCPauseMillis=50".into(),
                "-XX:G1HeapRegionSize=32M".into(),
            ],
            game_args: Vec::new(),
            environment: HashMap::new(),
            notes: String::new(),
            icon_key: "default".into(),
        }
    }
}

pub struct InstanceConfigManager {
    instances_dir: PathBuf,
}

impl InstanceConfigManager {
    pub fn new(instances_dir: &str) -> Self {
        InstanceConfigManager {
            instances_dir: PathBuf::from(instances_dir),
        }
    }

    pub fn create(&self, name: &str, config: InstanceConfig) -> Result<InstanceProfile> {
        let instance_path = self.instances_dir.join(sanitize_name(name));
        fs::create_dir_all(&instance_path)?;

        let config_path = instance_path.join("instance.json");
        let json = serde_json::to_string_pretty(&config).map_err(CoreError::Json)?;
        fs::write(&config_path, &json)?;

        Ok(InstanceProfile {
            config,
            path: instance_path,
            last_played: 0,
            total_play_time: 0,
            mod_count: 0,
            modpack_id: None,
            modpack_version: None,
            modpack_platform: None,
        })
    }

    pub fn load(&self, name: &str) -> Result<InstanceProfile> {
        let instance_path = self.instances_dir.join(sanitize_name(name));
        let config_path = instance_path.join("instance.json");

        if !config_path.exists() {
            return Err(CoreError::InvalidData(format!(
                "Instance '{}' not found",
                name
            )));
        }

        let content = fs::read_to_string(&config_path)?;
        let config: InstanceConfig = serde_json::from_str(&content).map_err(CoreError::Json)?;

        let profile_path = instance_path.join("profile.json");
        let profile: InstanceProfile = if profile_path.exists() {
            let profile_content = fs::read_to_string(&profile_path)?;
            serde_json::from_str(&profile_content).map_err(CoreError::Json)?
        } else {
            InstanceProfile {
                config: config.clone(),
                path: instance_path.clone(),
                last_played: 0,
                total_play_time: 0,
                mod_count: 0,
                modpack_id: None,
                modpack_version: None,
                modpack_platform: None,
            }
        };

        Ok(InstanceProfile { config, ..profile })
    }

    pub fn save(&self, profile: &InstanceProfile) -> Result<()> {
        fs::create_dir_all(&profile.path)?;

        let config_path = profile.path.join("instance.json");
        let json = serde_json::to_string_pretty(&profile.config).map_err(CoreError::Json)?;
        fs::write(&config_path, &json)?;

        let profile_path = profile.path.join("profile.json");
        let profile_json = serde_json::to_string_pretty(&serde_json::json!({
            "last_played": profile.last_played,
            "total_play_time": profile.total_play_time,
            "mod_count": profile.mod_count,
            "modpack_id": profile.modpack_id,
            "modpack_version": profile.modpack_version,
            "modpack_platform": profile.modpack_platform,
        }))
        .map_err(CoreError::Json)?;
        fs::write(&profile_path, &profile_json)?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>> {
        let mut instances = Vec::new();
        if !self.instances_dir.exists() {
            return Ok(instances);
        }

        for entry in fs::read_dir(&self.instances_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && path.join("instance.json").exists() {
                if let Some(name) = path.file_name() {
                    instances.push(name.to_string_lossy().to_string());
                }
            }
        }

        instances.sort();
        Ok(instances)
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        let instance_path = self.instances_dir.join(sanitize_name(name));
        if instance_path.exists() {
            fs::remove_dir_all(&instance_path)?;
        }
        Ok(())
    }

    pub fn duplicate(&self, source_name: &str, new_name: &str) -> Result<InstanceProfile> {
        let profile = self.load(source_name)?;
        let new_config = InstanceConfig {
            name: new_name.to_string(),
            ..profile.config
        };
        self.create(new_name, new_config)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.instances_dir
            .join(sanitize_name(name))
            .join("instance.json")
            .exists()
    }
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = InstanceConfig::default();
        assert_eq!(config.game_version, "latest_release");
        assert_eq!(config.memory.min_mb, 512);
        assert_eq!(config.memory.max_mb, 2048);
        assert_eq!(config.resolution.width, 854);
    }

    #[test]
    fn test_sanitize_name() {
        assert_eq!(sanitize_name("My Instance"), "My Instance");
        assert_eq!(sanitize_name("../evil"), "__evil");
        assert_eq!(sanitize_name("hello/world:test"), "hello_world_test");
    }

    #[test]
    fn test_serialize_roundtrip() {
        let config = InstanceConfig {
            name: "Test".into(),
            game_version: "1.21".into(),
            loader: LoaderConfig {
                loader_type: "forge".into(),
                loader_version: "50.0.0".into(),
            },
            memory: MemoryConfig {
                min_mb: 1024,
                max_mb: 4096,
            },
            resolution: ResolutionConfig {
                width: 1920,
                height: 1080,
                fullscreen: false,
            },
            java_args: vec!["-Xmx2G".into()],
            game_args: Vec::new(),
            environment: HashMap::new(),
            notes: "".into(),
            icon_key: "grass".into(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: InstanceConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "Test");
        assert_eq!(deserialized.game_version, "1.21");
        assert_eq!(deserialized.loader.loader_type, "forge");
        assert_eq!(deserialized.memory.min_mb, 1024);
        assert_eq!(deserialized.resolution.width, 1920);
    }
}
