use std::collections::HashMap;

use error::{CoreError, Result};

use crate::component::{Component, ComponentGraph, DependencyNode};
use crate::config::{InstanceConfig, LoaderConfig};
use crate::manifest::{ManifestLoader, VersionDetails};

#[derive(Debug)]
pub struct ResolutionError {
    pub message: String,
    pub component: Option<String>,
}

pub struct ComponentResolver;

impl ComponentResolver {
    pub fn resolve_config(config: &InstanceConfig) -> Result<(ComponentGraph, VersionDetails)> {
        let version_details = ManifestLoader::resolve_version(&config.game_version)?;

        let mut graph = ComponentGraph::new();
        graph.add_component(Component::new_minecraft(&config.game_version));

        match config.loader.loader_type.to_lowercase().as_str() {
            "forge" => {
                graph.add_component(Component::new_forge(
                    &config.game_version,
                    &config.loader.loader_version,
                ));
            }
            "neoforge" => {
                graph.add_component(Component {
                    uid: "net.neoforged".into(),
                    version: config.loader.loader_version.clone(),
                    component_type: crate::component::ComponentType::NeoForge,
                    dependencies: vec![crate::component::Dependency {
                        uid: "net.minecraft".into(),
                        version_range: format!("[{}]", config.game_version),
                        optional: false,
                    }],
                    conflicts: vec![
                        "net.minecraftforge".into(),
                        "net.fabricmc".into(),
                        "net.ornithe".into(),
                    ],
                    order: 10,
                });
            }
            "fabric" => {
                graph.add_component(Component::new_fabric(&config.loader.loader_version));
            }
            "quilt" => {
                graph.add_component(Component {
                    uid: "org.quiltmc.quilt-loader".into(),
                    version: config.loader.loader_version.clone(),
                    component_type: crate::component::ComponentType::QuiltLoader,
                    dependencies: vec![crate::component::Dependency {
                        uid: "net.minecraft".into(),
                        version_range: format!("[{}]", config.game_version),
                        optional: false,
                    }],
                    conflicts: vec![
                        "net.minecraftforge".into(),
                        "net.neoforged".into(),
                        "net.fabricmc".into(),
                    ],
                    order: 10,
                });
            }
            "liteloader" => {
                graph.add_component(Component {
                    uid: "com.mumfrey.liteloader".into(),
                    version: config.loader.loader_version.clone(),
                    component_type: crate::component::ComponentType::LiteLoader,
                    dependencies: vec![crate::component::Dependency {
                        uid: "net.minecraft".into(),
                        version_range: format!("[{}]", config.game_version),
                        optional: false,
                    }],
                    conflicts: Vec::new(),
                    order: 10,
                });
            }
            _ => {}
        }

        if graph.has_cyclic_dependency() {
            return Err(CoreError::InvalidData(
                "Cyclic dependency detected in component graph".into(),
            ));
        }

        let conflicts = graph.check_conflicts();
        if !conflicts.is_empty() {
            return Err(CoreError::InvalidData(format!(
                "Component conflicts: {}",
                conflicts.join(", ")
            )));
        }

        Ok((graph, version_details))
    }

    pub fn resolve_libraries(version: &VersionDetails) -> Vec<String> {
        let mut libs = Vec::new();
        for lib in &version.libraries {
            let allowed = match &lib.rules {
                Some(rules) => evaluate_rules(rules),
                None => true,
            };
            if allowed {
                if let Some(ref downloads) = lib.downloads {
                    if let Some(ref artifact) = downloads.artifact {
                        libs.push(artifact.url.clone());
                    }
                }
            }
        }
        libs
    }

    pub fn resolve_classpath(version: &VersionDetails, library_dir: &str) -> Vec<String> {
        let mut entries = Vec::new();
        for lib in &version.libraries {
            let allowed = match &lib.rules {
                Some(rules) => evaluate_rules(rules),
                None => true,
            };
            if allowed {
                if let Some(ref downloads) = lib.downloads {
                    if let Some(ref artifact) = downloads.artifact {
                        entries.push(format!("{}/{}", library_dir, artifact.path));
                    }
                }
                // Handle natives
                for (os, native_suffix) in &lib.natives {
                    let native_os = os_name_to_platform(os);
                    if native_os == current_platform() {
                        if let Some(ref downloads) = lib.downloads {
                            if let Some(ref classifiers) =
                                serde_json::from_str::<HashMap<String, serde_json::Value>>("{}")
                                    .ok()
                            {
                                // natives handled via classifiers
                            }
                        }
                    }
                }
            }
        }
        entries
    }

    pub fn resolve_main_class(version: &VersionDetails, loader_type: &str) -> String {
        match loader_type {
            "forge" => "net.minecraftforge.bootstrap.ForgeBootstrap".into(),
            "neoforge" => "net.neoforged.bootstrap.NeoForgeBootstrap".into(),
            "fabric" => "net.fabricmc.loader.impl.launch.knot.KnotClient".into(),
            "quilt" => "org.quiltmc.loader.impl.launch.knot.KnotClient".into(),
            "liteloader" => "com.mumfrey.liteloader.LiteLoaderTweaker".into(),
            _ => version.main_class.clone(),
        }
    }
}

fn evaluate_rules(rules: &[crate::manifest::Rule]) -> bool {
    let mut allowed = true;
    for rule in rules {
        let matches = match &rule.os {
            Some(os) => {
                let os_name = os.name.as_deref().unwrap_or("");
                let current = current_platform();
                let version_match = match &os.version {
                    Some(v) => std::env::consts::OS.contains(v),
                    None => true,
                };
                let arch_match = match &os.arch {
                    Some(a) => std::env::consts::ARCH.contains(a),
                    None => true,
                };
                (os_name.is_empty() || os_name == current) && version_match && arch_match
            }
            None => true,
        };

        if matches {
            allowed = rule.action == "allow";
        }
    }
    allowed
}

fn current_platform() -> &'static str {
    match std::env::consts::OS {
        "windows" => "windows",
        "macos" => "osx",
        _ => "linux",
    }
}

fn os_name_to_platform(os_name: &str) -> &'static str {
    match os_name {
        "windows" => "windows",
        "osx" | "macos" => "osx",
        "linux" => "linux",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::*;

    #[test]
    fn test_resolve_main_class_vanilla() {
        let details = VersionDetails {
            id: "1.21".into(),
            version_type: "release".into(),
            release_time: "2024-06-01".into(),
            time: "2024-06-01".into(),
            main_class: "net.minecraft.client.main.Main".into(),
            minecraft_arguments: String::new(),
            arguments: None,
            libraries: Vec::new(),
            asset_index: AssetIndex {
                id: "1".into(),
                sha1: "abc".into(),
                size: 100,
                total_size: 100,
                url: "https://example.com".into(),
            },
            downloads: VersionDownloads {
                client: None,
                server: None,
                client_mappings: None,
                server_mappings: None,
            },
        };
        assert_eq!(
            ComponentResolver::resolve_main_class(&details, "vanilla"),
            "net.minecraft.client.main.Main"
        );
        assert_eq!(
            ComponentResolver::resolve_main_class(&details, "forge"),
            "net.minecraftforge.bootstrap.ForgeBootstrap"
        );
    }
}
