use thirtyfour::prelude::*;

use crate::core::settings::*;

// build ac function to initialize the webdriver with option to run headless, default to true
async fn initialize_webdriver(headless: Option<bool>) -> WebDriverResult<WebDriver> {
    let mut caps = DesiredCapabilities::chrome();
    let settings: &Settings = app_settings();
    if headless.unwrap_or(false) {
        caps.add_arg("--headless")?;
        // caps.add_chrome_arg("--disable-gpu")?;
        // caps.add_chrome_arg("--no-sandbox")?;
    }
    let driver = WebDriver::new(settings.webdriver_url.clone(), caps).await?;
    Ok(driver)
}
