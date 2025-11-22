use std::path::Path;

use dotenv::dotenv;

use lazy_static::lazy_static;

#[allow(dead_code)]
pub struct Settings {
    pub development_mode: bool,
    pub webdriver_url: String,
    pub root_path: String,
    pub yolo_image_path: String,
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        dotenv().ok();

        let development_mode = true;
        let webdriver_url = "http://localhost:9515";
        let root_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        let yolo_image_path = format!("{}/dal/local/dataset/image", root_path);

        Settings {
            development_mode,
            webdriver_url: webdriver_url.to_string(),
            root_path,
            yolo_image_path,
        }
    };
}

#[allow(dead_code)]
pub fn app_settings() -> &'static Settings {
    &SETTINGS
}
