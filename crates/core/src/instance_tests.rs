use crate::ffi::instance_generator::{create_instance, InstanceConfig, InstanceType};
use std::fs;
use std::path::Path;

macro_rules! generate_instance_test {
    ($test_name:ident, $type_enum:expr, $type_str:expr, $name_prefix:expr, $version:expr, $key_str:expr) => {
        #[test]
        fn $test_name() {
            let base_path = Path::new("tests/test_instances");
            let _ = fs::create_dir_all(base_path);

            let config = InstanceConfig {
                instance_type: $type_enum,
                version_or_modpack: $version.to_string(),
            };

            let instance_name = stringify!($test_name);
            create_instance(base_path, instance_name, config).unwrap();

            let config_path = base_path.join(instance_name).join("instance.cfg");
            assert!(
                config_path.exists(),
                "Config path does not exist for {}",
                instance_name
            );

            let content = fs::read_to_string(&config_path).unwrap();
            assert!(
                content.contains($type_str),
                "Missing type {} in {}",
                $type_str,
                instance_name
            );
            assert!(
                content.contains($version),
                "Missing version {} in {}",
                $version,
                instance_name
            );
            assert!(
                content.contains($key_str),
                "Missing key {} in {}",
                $key_str,
                instance_name
            );
        }
    };
}

// Generate 100+ tests!
// Vanilla versions
generate_instance_test!(
    test_vanilla_1_19_2,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.19.2",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_18_2,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.18.2",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_17_1,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.17.1",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_16_5,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.16.5",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_15_2,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.15.2",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_14_4,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.14.4",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_13_2,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.13.2",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_12_2,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.12.2",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_8_9,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.8.9",
    "Version"
);
generate_instance_test!(
    test_vanilla_1_7_10,
    InstanceType::Vanilla,
    "Vanilla",
    "vanilla",
    "1.7.10",
    "Version"
);

// Forge versions
generate_instance_test!(
    test_forge_1_19_2,
    InstanceType::Forge,
    "Forge",
    "forge",
    "43.2.0",
    "Version"
);
generate_instance_test!(
    test_forge_1_18_2,
    InstanceType::Forge,
    "Forge",
    "forge",
    "40.1.0",
    "Version"
);
generate_instance_test!(
    test_forge_1_16_5,
    InstanceType::Forge,
    "Forge",
    "forge",
    "36.2.39",
    "Version"
);
generate_instance_test!(
    test_forge_1_12_2,
    InstanceType::Forge,
    "Forge",
    "forge",
    "14.23.5.2859",
    "Version"
);
generate_instance_test!(
    test_forge_1_8_9,
    InstanceType::Forge,
    "Forge",
    "forge",
    "11.15.1.2318",
    "Version"
);
generate_instance_test!(
    test_forge_1_7_10,
    InstanceType::Forge,
    "Forge",
    "forge",
    "10.13.4.1614",
    "Version"
);
generate_instance_test!(
    test_forge_latest,
    InstanceType::Forge,
    "Forge",
    "forge",
    "latest",
    "Version"
);
generate_instance_test!(
    test_forge_recommended,
    InstanceType::Forge,
    "Forge",
    "forge",
    "recommended",
    "Version"
);

// Fabric versions
generate_instance_test!(
    test_fabric_1_20,
    InstanceType::Fabric,
    "Fabric",
    "fabric",
    "0.14.21",
    "Version"
);
generate_instance_test!(
    test_fabric_1_19_4,
    InstanceType::Fabric,
    "Fabric",
    "fabric",
    "0.14.19",
    "Version"
);
generate_instance_test!(
    test_fabric_1_19_2,
    InstanceType::Fabric,
    "Fabric",
    "fabric",
    "0.14.9",
    "Version"
);
generate_instance_test!(
    test_fabric_1_18_2,
    InstanceType::Fabric,
    "Fabric",
    "fabric",
    "0.13.3",
    "Version"
);
generate_instance_test!(
    test_fabric_1_17_1,
    InstanceType::Fabric,
    "Fabric",
    "fabric",
    "0.11.7",
    "Version"
);
generate_instance_test!(
    test_fabric_1_16_5,
    InstanceType::Fabric,
    "Fabric",
    "fabric",
    "0.11.3",
    "Version"
);

// Quilt versions
generate_instance_test!(
    test_quilt_1_20,
    InstanceType::Quilt,
    "Quilt",
    "quilt",
    "0.19.0",
    "Version"
);
generate_instance_test!(
    test_quilt_1_19_2,
    InstanceType::Quilt,
    "Quilt",
    "quilt",
    "0.18.1",
    "Version"
);
generate_instance_test!(
    test_quilt_1_18_2,
    InstanceType::Quilt,
    "Quilt",
    "quilt",
    "0.17.0",
    "Version"
);

// LiteLoader versions
generate_instance_test!(
    test_liteloader_1_12_2,
    InstanceType::LiteLoader,
    "LiteLoader",
    "liteloader",
    "1.12.2-00-SNAPSHOT",
    "Version"
);
generate_instance_test!(
    test_liteloader_1_8_9,
    InstanceType::LiteLoader,
    "LiteLoader",
    "liteloader",
    "1.8.9-00-SNAPSHOT",
    "Version"
);

// FTB Modpacks
generate_instance_test!(
    test_ftb_direwolf20,
    InstanceType::FTB,
    "FTB",
    "ftb",
    "Direwolf20",
    "Modpack"
);
generate_instance_test!(
    test_ftb_revelation,
    InstanceType::FTB,
    "FTB",
    "ftb",
    "FTB_Revelation",
    "Modpack"
);
generate_instance_test!(
    test_ftb_interactions,
    InstanceType::FTB,
    "FTB",
    "ftb",
    "FTB_Interactions",
    "Modpack"
);
generate_instance_test!(
    test_ftb_skyfactory_3,
    InstanceType::FTB,
    "FTB",
    "ftb",
    "SkyFactory3",
    "Modpack"
);
generate_instance_test!(
    test_ftb_infinity_evolved,
    InstanceType::FTB,
    "FTB",
    "ftb",
    "InfinityEvolved",
    "Modpack"
);
generate_instance_test!(
    test_ftb_stoneblock_2,
    InstanceType::FTB,
    "FTB",
    "ftb",
    "Stoneblock2",
    "Modpack"
);

// Tekkit Modpacks
generate_instance_test!(
    test_tekkit_classic,
    InstanceType::Tekkit,
    "Tekkit",
    "tekkit",
    "TekkitClassic",
    "Modpack"
);
generate_instance_test!(
    test_tekkit_legends,
    InstanceType::Tekkit,
    "Tekkit",
    "tekkit",
    "TekkitLegends",
    "Modpack"
);
generate_instance_test!(
    test_tekkit_lite,
    InstanceType::Tekkit,
    "Tekkit",
    "tekkit",
    "TekkitLite",
    "Modpack"
);
generate_instance_test!(
    test_hexxit,
    InstanceType::Tekkit,
    "Tekkit",
    "tekkit",
    "Hexxit",
    "Modpack"
);
generate_instance_test!(
    test_voltz,
    InstanceType::Tekkit,
    "Tekkit",
    "tekkit",
    "Voltz",
    "Modpack"
);

// CurseForge Modpacks
generate_instance_test!(
    test_curse_rlcraft,
    InstanceType::CurseForge,
    "CurseForge",
    "curseforge",
    "RLCraft",
    "Modpack"
);
generate_instance_test!(
    test_curse_sevtech,
    InstanceType::CurseForge,
    "CurseForge",
    "curseforge",
    "SevTech_Ages",
    "Modpack"
);
generate_instance_test!(
    test_curse_all_the_mods_8,
    InstanceType::CurseForge,
    "CurseForge",
    "curseforge",
    "AllTheMods8",
    "Modpack"
);
generate_instance_test!(
    test_curse_pixelmon,
    InstanceType::CurseForge,
    "CurseForge",
    "curseforge",
    "Pixelmon",
    "Modpack"
);
generate_instance_test!(
    test_curse_valhelsia_3,
    InstanceType::CurseForge,
    "CurseForge",
    "curseforge",
    "Valhelsia3",
    "Modpack"
);

// Modrinth Modpacks
generate_instance_test!(
    test_modrinth_fabulously_optimized,
    InstanceType::Modrinth,
    "Modrinth",
    "modrinth",
    "Fabulously_Optimized",
    "Modpack"
);
generate_instance_test!(
    test_modrinth_sop,
    InstanceType::Modrinth,
    "Modrinth",
    "modrinth",
    "Simply_Optimized",
    "Modpack"
);
generate_instance_test!(
    test_modrinth_additive,
    InstanceType::Modrinth,
    "Modrinth",
    "modrinth",
    "Additive",
    "Modpack"
);
generate_instance_test!(
    test_modrinth_vulbo,
    InstanceType::Modrinth,
    "Modrinth",
    "modrinth",
    "Vulbo",
    "Modpack"
);

// ATLauncher Modpacks
generate_instance_test!(
    test_atl_yogscast,
    InstanceType::ATLauncher,
    "ATLauncher",
    "atl",
    "Yogscast_Complete",
    "Modpack"
);
generate_instance_test!(
    test_atl_skyfactory_4,
    InstanceType::ATLauncher,
    "ATLauncher",
    "atl",
    "SkyFactory4",
    "Modpack"
);
generate_instance_test!(
    test_atl_crundee_craft,
    InstanceType::ATLauncher,
    "ATLauncher",
    "atl",
    "CrundeeCraft",
    "Modpack"
);

// Using a macro loop to easily generate up to 100+ tests
macro_rules! bulk_vanilla_tests {
    ($($ver:ident => $ver_str:expr),*) => {
        $(
            generate_instance_test!($ver, InstanceType::Vanilla, "Vanilla", "vanilla", $ver_str, "Version");
        )*
    }
}

bulk_vanilla_tests!(
    test_vanilla_1_0 => "1.0",
    test_vanilla_1_1 => "1.1",
    test_vanilla_1_2 => "1.2",
    test_vanilla_1_2_5 => "1.2.5",
    test_vanilla_1_3_2 => "1.3.2",
    test_vanilla_1_4_7 => "1.4.7",
    test_vanilla_1_5_2 => "1.5.2",
    test_vanilla_1_6_4 => "1.6.4",
    test_vanilla_1_7_2 => "1.7.2",
    test_vanilla_1_8 => "1.8",
    test_vanilla_1_9 => "1.9",
    test_vanilla_1_9_4 => "1.9.4",
    test_vanilla_1_10 => "1.10",
    test_vanilla_1_10_2 => "1.10.2",
    test_vanilla_1_11 => "1.11",
    test_vanilla_1_11_2 => "1.11.2",
    test_vanilla_1_12 => "1.12",
    test_vanilla_1_12_1 => "1.12.1",
    test_vanilla_1_13 => "1.13",
    test_vanilla_1_13_1 => "1.13.1",
    test_vanilla_1_14 => "1.14",
    test_vanilla_1_14_1 => "1.14.1",
    test_vanilla_1_14_2 => "1.14.2",
    test_vanilla_1_14_3 => "1.14.3",
    test_vanilla_1_15 => "1.15",
    test_vanilla_1_15_1 => "1.15.1",
    test_vanilla_1_16 => "1.16",
    test_vanilla_1_16_1 => "1.16.1",
    test_vanilla_1_16_2 => "1.16.2",
    test_vanilla_1_16_3 => "1.16.3",
    test_vanilla_1_16_4 => "1.16.4",
    test_vanilla_1_17 => "1.17",
    test_vanilla_1_18 => "1.18",
    test_vanilla_1_18_1 => "1.18.1",
    test_vanilla_1_19 => "1.19",
    test_vanilla_1_19_1 => "1.19.1",
    test_vanilla_1_19_3 => "1.19.3",
    test_vanilla_1_19_4 => "1.19.4",
    test_vanilla_1_20_1 => "1.20.1",
    test_vanilla_1_20_2 => "1.20.2",
    test_vanilla_1_20_3 => "1.20.3",
    test_vanilla_1_20_4 => "1.20.4",
    test_vanilla_1_20_5 => "1.20.5",
    test_vanilla_1_20_6 => "1.20.6",
    test_vanilla_1_21 => "1.21"
);
