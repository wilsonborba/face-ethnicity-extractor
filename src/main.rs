use std::vec;

use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .get("https://generated.photos/faces/adult/asian-race")
        .await?;

    // wait a bit for overlays/ads to load
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    //let button = driver.find(By::Css("button.loadmore-btn")).await?;

    // Optionally scroll near the bottom
    driver
        .execute_script("window.scrollTo(0, document.body.scrollHeight);", vec![])
        .await?;

    // JS click the button directly – no hit-test
    driver
        .execute_script(
            "document.querySelector('button.loadmore-btn')?.click();",
            vec![],
        )
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    for el in driver.find_all(By::ClassName("card-image")).await? {
        let val = el.find_all(By::Tag("a")).await?;
        for v in val {
            let img = v.find(By::Tag("img")).await?;
            let href = img.attr("src").await?;
            println!("Found href: {:?}", href);
        }
    }

    driver.quit().await?;
    Ok(())
}
