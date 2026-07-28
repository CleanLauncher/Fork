use crate::models::JavaProcessConfig;
use std::process::{Command, Stdio};

pub struct Launcher {
    config: JavaProcessConfig,
}

impl Launcher {
    pub fn new(config: JavaProcessConfig) -> Self {
        Self { config }
    }

    pub fn launch(&self) -> Result<(), String> {
        log::info!("Launching Java process from: {}", self.config.working_dir);
        
        let mut command = Command::new(&self.config.java_path);
        command.current_dir(&self.config.working_dir);
        
        // Add JVM arguments
        for arg in &self.config.jvm_args {
            command.arg(arg);
        }
        
        // Add Main class
        command.arg(&self.config.main_class);
        
        // Add Game arguments
        for arg in &self.config.game_args {
            command.arg(arg);
        }
        
        // Set Environment
        for (key, value) in &self.config.environment {
            command.env(key, value);
        }
        
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        match command.spawn() {
            Ok(_) => {
                log::info!("Process successfully started.");
                Ok(())
            },
            Err(e) => {
                log::error!("Failed to launch process: {}", e);
                Err(format!("Process launch failed: {}", e))
            }
        }
    }
}
