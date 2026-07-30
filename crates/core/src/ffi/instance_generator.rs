use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub enum InstanceType {
    Vanilla,
    FTB,
    Tekkit,
    Forge,
    Fabric,
    Quilt,
    LiteLoader,
    CurseForge,
    Modrinth,
    ATLauncher,
}

#[derive(Clone, Debug)]
pub struct InstanceConfig {
    pub instance_type: InstanceType,
    pub version_or_modpack: String,
}

pub fn create_instance(
    base_path: &Path,
    name: &str,
    config: InstanceConfig,
) -> Result<(), std::io::Error> {
    let instance_dir = base_path.join(name);
    fs::create_dir_all(&instance_dir)?;

    let config_path = instance_dir.join("instance.cfg");

    let type_str = match config.instance_type {
        InstanceType::Vanilla => "Vanilla",
        InstanceType::FTB => "FTB",
        InstanceType::Tekkit => "Tekkit",
        InstanceType::Forge => "Forge",
        InstanceType::Fabric => "Fabric",
        InstanceType::Quilt => "Quilt",
        InstanceType::LiteLoader => "LiteLoader",
        InstanceType::CurseForge => "CurseForge",
        InstanceType::Modrinth => "Modrinth",
        InstanceType::ATLauncher => "ATLauncher",
    };

    let key = match config.instance_type {
        InstanceType::Vanilla
        | InstanceType::Forge
        | InstanceType::Fabric
        | InstanceType::Quilt
        | InstanceType::LiteLoader => "Version",
        _ => "Modpack",
    };

    let config_content = format!(
        "InstanceType={}\n{}={}\n",
        type_str, key, config.version_or_modpack
    );
    fs::write(config_path, config_content)?;

    Ok(())
}
