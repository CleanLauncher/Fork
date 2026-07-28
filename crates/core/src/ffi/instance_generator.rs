use std::fs;
use std::path::Path;

pub enum InstanceType {
    Vanilla,
    FTB,
    Tekkit,
}

pub struct InstanceConfig {
    pub instance_type: InstanceType,
    pub version_or_modpack: String,
}

pub fn create_instance(base_path: &Path, name: &str, config: InstanceConfig) -> Result<(), std::io::Error> {
    let instance_dir = base_path.join(name);
    fs::create_dir_all(&instance_dir)?;
    
    let config_path = instance_dir.join("instance.cfg");
    
    let type_str = match config.instance_type {
        InstanceType::Vanilla => "Vanilla",
        InstanceType::FTB => "FTB",
        InstanceType::Tekkit => "Tekkit",
    };
    
    let key = match config.instance_type {
        InstanceType::Vanilla => "Version",
        _ => "Modpack",
    };
    
    let config_content = format!("InstanceType={}\n{}={}\n", type_str, key, config.version_or_modpack);
    fs::write(config_path, config_content)?;
    
    Ok(())
}
