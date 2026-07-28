fn main() {
    cxx_build::bridge("src/cxx_bridge.rs")
        .flag_if_supported("-std=c++17")
        .compile("core_cxx_bridge");

    println!("cargo:rerun-if-changed=src/cxx_bridge.rs");
}
