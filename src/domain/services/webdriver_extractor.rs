use thirtyfour::prelude::*;

pub async fn scroll_down_page(
    driver: &thirtyfour::WebDriver,
    await_for: u64,
) -> WebDriverResult<()> {
    tokio::time::sleep(std::time::Duration::from_secs(await_for)).await;
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
    Ok(())
}

pub async fn extract_image_urls(driver: &thirtyfour::WebDriver) -> WebDriverResult<Vec<String>> {
    let mut image_urls: Vec<String> = Vec::new();

    for el in driver.find_all(By::ClassName("card-image")).await? {
        let val = el.find_all(By::Tag("a")).await?;
        for v in val {
            let img = v.find(By::Tag("img")).await?;
            if let Some(href) = img.attr("src").await? {
                image_urls.push(href);
            }
        }
    }

    Ok(image_urls)
}

pub async fn download_image_to_path(
    driver: &thirtyfour::WebDriver,
    image_url: &str,
    download_path: &str,
) -> WebDriverResult<()> {
    let script = format!(
        r#"
        var link = document.createElement('a');
        link.href = '{}';
        link.download = '{}';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    "#,
        image_url, download_path
    );

    driver.execute_script(&script, vec![]).await?;
    Ok(())
}
