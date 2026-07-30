use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub uid: String,
    pub version: String,
    pub component_type: ComponentType,
    pub dependencies: Vec<Dependency>,
    pub conflicts: Vec<String>,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Minecraft,
    Forge,
    NeoForge,
    Fabric,
    FabricLoader,
    Quilt,
    QuiltLoader,
    LiteLoader,
    Mod,
    ResourcePack,
    ShaderPack,
    Library,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub uid: String,
    pub version_range: String,
    pub optional: bool,
}

#[derive(Debug, Default, Clone)]
pub struct DependencyNode {
    pub component: Component,
    pub children: Vec<DependencyNode>,
    pub resolved: bool,
}

#[derive(Debug, Default)]
pub struct ComponentGraph {
    pub components: HashMap<String, DependencyNode>,
}

impl ComponentGraph {
    pub fn new() -> Self {
        ComponentGraph {
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        let uid = component.uid.clone();
        self.components.insert(
            uid,
            DependencyNode {
                component,
                children: Vec::new(),
                resolved: false,
            },
        );
    }

    pub fn get(&self, uid: &str) -> Option<&Component> {
        self.components.get(uid).map(|n| &n.component)
    }

    pub fn get_mut(&mut self, uid: &str) -> Option<&mut DependencyNode> {
        self.components.get_mut(uid)
    }

    pub fn resolve_order(&self) -> Vec<String> {
        let mut visited = std::collections::HashSet::new();
        let mut order = Vec::new();

        fn dfs(
            uid: &str,
            graph: &ComponentGraph,
            visited: &mut std::collections::HashSet<String>,
            order: &mut Vec<String>,
            path: &mut std::collections::HashSet<String>,
        ) {
            if !visited.contains(uid) {
                visited.insert(uid.to_string());
                path.insert(uid.to_string());

                if let Some(node) = graph.components.get(uid) {
                    for dep in &node.component.dependencies {
                        if !dep.optional && !path.contains(&dep.uid) {
                            dfs(&dep.uid, graph, visited, order, path);
                        }
                    }
                }

                path.remove(uid);
                order.push(uid.to_string());
            }
        }

        let mut path = std::collections::HashSet::new();
        for uid in self.components.keys() {
            dfs(uid, self, &mut visited, &mut order, &mut path);
        }

        order
    }

    pub fn has_cyclic_dependency(&self) -> bool {
        fn dfs(
            uid: &str,
            graph: &ComponentGraph,
            visited: &mut std::collections::HashSet<String>,
            stack: &mut std::collections::HashSet<String>,
        ) -> bool {
            if stack.contains(uid) {
                return true;
            }
            if visited.contains(uid) {
                return false;
            }
            visited.insert(uid.to_string());
            stack.insert(uid.to_string());

            if let Some(node) = graph.components.get(uid) {
                for dep in &node.component.dependencies {
                    if dfs(&dep.uid, graph, visited, stack) {
                        return true;
                    }
                }
            }

            stack.remove(uid);
            false
        }

        let mut visited = std::collections::HashSet::new();
        let mut stack = std::collections::HashSet::new();

        for uid in self.components.keys() {
            if dfs(uid, self, &mut visited, &mut stack) {
                return true;
            }
        }
        false
    }

    pub fn check_conflicts(&self) -> Vec<String> {
        let mut conflicts = Vec::new();
        let uid_set: std::collections::HashSet<String> = self.components.keys().cloned().collect();

        for (uid, node) in &self.components {
            for conflict_uid in &node.component.conflicts {
                if uid_set.contains(conflict_uid) {
                    conflicts.push(format!("{} conflicts with {}", uid, conflict_uid));
                }
            }
        }
        conflicts
    }
}

impl Component {
    pub fn new_minecraft(version: &str) -> Self {
        Component {
            uid: "net.minecraft".into(),
            version: version.to_string(),
            component_type: ComponentType::Minecraft,
            dependencies: Vec::new(),
            conflicts: Vec::new(),
            order: 0,
        }
    }

    pub fn new_forge(mc_version: &str, forge_version: &str) -> Self {
        Component {
            uid: "net.minecraftforge".into(),
            version: format!("{}-{}", mc_version, forge_version),
            component_type: ComponentType::Forge,
            dependencies: vec![Dependency {
                uid: "net.minecraft".into(),
                version_range: format!("[{}]", mc_version),
                optional: false,
            }],
            conflicts: vec!["net.fabricmc".into(), "net.neoforged".into()],
            order: 10,
        }
    }

    pub fn new_fabric(loader_version: &str) -> Self {
        Component {
            uid: "net.fabricmc.fabric-loader".into(),
            version: loader_version.to_string(),
            component_type: ComponentType::FabricLoader,
            dependencies: vec![Dependency {
                uid: "net.minecraft".into(),
                version_range: "*".into(),
                optional: false,
            }],
            conflicts: vec![
                "net.minecraftforge".into(),
                "net.neoforged".into(),
                "net.ornithe".into(),
            ],
            order: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let mc = Component::new_minecraft("1.21");
        assert_eq!(mc.uid, "net.minecraft");
        assert_eq!(mc.component_type, ComponentType::Minecraft);
    }

    #[test]
    fn test_component_graph_resolve_order() {
        let mut graph = ComponentGraph::new();
        graph.add_component(Component::new_minecraft("1.21"));
        graph.add_component(Component::new_forge("1.21", "50.0.0"));

        let order = graph.resolve_order();
        assert_eq!(order.len(), 2);
        // Minecraft should be resolved before Forge (dependency)
        let mc_pos = order.iter().position(|u| u == "net.minecraft").unwrap();
        let forge_pos = order
            .iter()
            .position(|u| u == "net.minecraftforge")
            .unwrap();
        assert!(mc_pos < forge_pos);
    }

    #[test]
    fn test_cyclic_dependency_detection() {
        let mut graph = ComponentGraph::new();
        graph.add_component(Component {
            uid: "a".into(),
            version: "1".into(),
            component_type: ComponentType::Library,
            dependencies: vec![Dependency {
                uid: "b".into(),
                version_range: "*".into(),
                optional: false,
            }],
            conflicts: Vec::new(),
            order: 0,
        });
        graph.add_component(Component {
            uid: "b".into(),
            version: "1".into(),
            component_type: ComponentType::Library,
            dependencies: vec![Dependency {
                uid: "a".into(),
                version_range: "*".into(),
                optional: false,
            }],
            conflicts: Vec::new(),
            order: 0,
        });

        assert!(graph.has_cyclic_dependency());
    }

    #[test]
    fn test_conflict_detection() {
        let mut graph = ComponentGraph::new();
        graph.add_component(Component::new_forge("1.21", "50.0.0"));
        graph.add_component(Component::new_fabric("0.16.0"));

        let conflicts = graph.check_conflicts();
        assert!(!conflicts.is_empty());
    }
}
