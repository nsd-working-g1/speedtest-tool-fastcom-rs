use super::model;
use playwright::api;
use std::path::PathBuf;

pub async fn speedtest() -> Result<model::SpeedTestResultValues, playwright::Error> {
    log::debug!("testing network speed.");

    log::info!("------------- [CAUTION] -----------");
    log::info!("Trial implement.");
    log::info!("Launch browser by playwright-rust.");
    log::info!("And capture full page screenshot.");
    log::info!("------------- [CAUTION] -----------");

    let result: model::SpeedTestResultValues = model::SpeedTestResultValues {
        tested_datetime: chrono::Local::now(),
        download_speed_mega_bps: 0.0,
        upload_speed_mega_bps: 0.0,
    };

    let playwright: playwright::Playwright = playwright::Playwright::initialize().await?;
    playwright.prepare()?;
    let chromium: api::BrowserType = playwright.chromium();
    let browser: api::Browser = chromium.launcher().headless(true).launch().await?;
    let context: api::BrowserContext = browser.context_builder().build().await?;
    let page: api::Page = context.new_page().await?;
    page.goto_builder("https://google.com/").goto().await?;

    page.screenshot_builder()
        .path(PathBuf::from("./log/screenshot.png"))
        .screenshot()
        .await?;

    log::debug!("tested network speed.");

    Ok(result)
}
