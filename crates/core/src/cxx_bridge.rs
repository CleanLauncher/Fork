#[cxx::bridge]
pub mod ffi {
    struct AppState {
        version: String,
        is_online: bool,
    }

    extern "Rust" {
        type AppController;
        fn new_app_controller() -> Box<AppController>;
        fn get_state(self: &AppController) -> AppState;
        fn set_online(self: &mut AppController, online: bool);
    }
}

pub struct AppController {
    version: String,
    is_online: bool,
}

pub fn new_app_controller() -> Box<AppController> {
    Box::new(AppController {
        version: env!("CARGO_PKG_VERSION").to_string(),
        is_online: false,
    })
}

impl AppController {
    pub fn get_state(&self) -> ffi::AppState {
        ffi::AppState {
            version: self.version.clone(),
            is_online: self.is_online,
        }
    }

    pub fn set_online(&mut self, online: bool) {
        self.is_online = online;
    }
}
