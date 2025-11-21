use dotenv::dotenv;

use lazy_static::lazy_static;

#[allow(dead_code)]
pub struct Settings {
    pub development_mode: bool,
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        dotenv().ok();

        let development_mode = true;

        Settings { development_mode }
    };
}

#[allow(dead_code)]
pub fn app_settings() -> &'static Settings {
    &SETTINGS
}
