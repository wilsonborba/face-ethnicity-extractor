use core::settings::app_settings;
use tokio;

mod core;
mod dal;
mod domain;

#[tokio::main]
async fn main() {
    let root_path = &app_settings().root_path;

    let progress_file_path = format!("{}/dal/local/progress.json", root_path);

    debug!("Application started at root path: {}", root_path);
    debug!("Progress file path: {}", progress_file_path);
}
