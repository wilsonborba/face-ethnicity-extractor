use core::settings::app_settings;
use tokio;

mod core;
mod dal;
mod domain;

#[tokio::main]
async fn main() {
    debug!(
        "Application started at root path: {}",
        app_settings().root_path
    );
}
