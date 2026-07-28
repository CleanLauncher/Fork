pub mod models;
pub mod launcher;

pub use models::JavaProcessConfig;
pub use launcher::Launcher;

pub fn launch_game(config: JavaProcessConfig) -> Result<(), String> {
    let launcher = Launcher::new(config);
    launcher.launch()
}
