use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaProcessConfig {
    pub java_path: String,
    pub jvm_args: Vec<String>,
    pub main_class: String,
    pub game_args: Vec<String>,
    pub environment: std::collections::HashMap<String, String>,
    pub working_dir: String,
}
