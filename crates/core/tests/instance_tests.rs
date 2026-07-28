use std::fs;
use std::path::Path;
use core::ffi::instance_generator::{create_instance, InstanceConfig, InstanceType};

#[test]
fn test_create_vanilla_instance() {
    let base_path = Path::new("tests/test_instances");
    let _ = fs::create_dir_all(base_path);
    
    let config = InstanceConfig {
        instance_type: InstanceType::Vanilla,
        version_or_modpack: "1.19.2".to_string(),
    };
    create_instance(base_path, "vanilla", config).unwrap();
    
    let config_path = base_path.join("vanilla").join("instance.cfg");
    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("Vanilla"));
    assert!(content.contains("1.19.2"));
}

#[test]
fn test_create_ftb_instance() {
    let base_path = Path::new("tests/test_instances");
    let _ = fs::create_dir_all(base_path);
    
    let config = InstanceConfig {
        instance_type: InstanceType::FTB,
        version_or_modpack: "Direwolf20".to_string(),
    };
    create_instance(base_path, "ftb", config).unwrap();
    
    let config_path = base_path.join("ftb").join("instance.cfg");
    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("FTB"));
    assert!(content.contains("Direwolf20"));
}

#[test]
fn test_create_tekkit_instance() {
    let base_path = Path::new("tests/test_instances");
    let _ = fs::create_dir_all(base_path);
    
    let config = InstanceConfig {
        instance_type: InstanceType::Tekkit,
        version_or_modpack: "TekkitClassic".to_string(),
    };
    create_instance(base_path, "tekkit", config).unwrap();
    
    let config_path = base_path.join("tekkit").join("instance.cfg");
    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("Tekkit"));
    assert!(content.contains("TekkitClassic"));
}
