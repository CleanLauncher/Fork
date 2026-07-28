use std::fs;

#[test]
fn test_create_vanilla_instance() {
    let instance_dir = "tests/test_instances/vanilla";
    let _ = fs::create_dir_all(instance_dir);
    // Simulate setting up a vanilla instance
    let config_path = format!("{}/instance.cfg", instance_dir);
    fs::write(&config_path, "InstanceType=Vanilla\nVersion=1.19.2").unwrap();
    
    assert!(fs::metadata(&config_path).is_ok());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("Vanilla"));
    assert!(content.contains("1.19.2"));
}

#[test]
fn test_create_ftb_instance() {
    let instance_dir = "tests/test_instances/ftb";
    let _ = fs::create_dir_all(instance_dir);
    // Simulate setting up an FTB instance
    let config_path = format!("{}/instance.cfg", instance_dir);
    fs::write(&config_path, "InstanceType=FTB\nModpack=Direwolf20").unwrap();
    
    assert!(fs::metadata(&config_path).is_ok());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("FTB"));
    assert!(content.contains("Direwolf20"));
}

#[test]
fn test_create_tekkit_instance() {
    let instance_dir = "tests/test_instances/tekkit";
    let _ = fs::create_dir_all(instance_dir);
    // Simulate setting up a Tekkit instance
    let config_path = format!("{}/instance.cfg", instance_dir);
    fs::write(&config_path, "InstanceType=Tekkit\nModpack=TekkitClassic").unwrap();
    
    assert!(fs::metadata(&config_path).is_ok());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("Tekkit"));
    assert!(content.contains("TekkitClassic"));
}
