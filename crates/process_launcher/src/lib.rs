use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchProfile {
    pub java_path: String,
    pub main_class: String,
    pub args: Vec<String>,
    pub env_vars: std::collections::HashMap<String, String>,
}

pub fn launch_game(profile: &LaunchProfile) -> Result<std::process::Child, String> {
    log::info!("Launching game with main class: {}", profile.main_class);
    
    let mut cmd = Command::new(&profile.java_path);
    
    cmd.args(&profile.args);
    cmd.envs(&profile.env_vars);
    
    cmd.spawn().map_err(|e| e.to_string())
}
