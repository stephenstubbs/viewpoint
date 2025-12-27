//! Basic navigation example demonstrating `Viewpoint`'s browser automation.
//!
//! This example launches a headless Chromium browser, navigates to a URL,
//! and demonstrates different wait states.
//!
//! # Running
//!
//! Make sure Chromium is installed and accessible, then run:
//!
//! ```sh
//! cargo run --example basic_navigation
//! ```

use std::time::Duration;

use viewpoint_core::{Browser, DocumentLoadState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching browser...");

    // Launch a headless browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await?;

    println!("Browser launched successfully!");

    // Create a new browser context (isolated environment)
    let context = browser.new_context().await?;
    println!("Created browser context: {}", context.id());

    // Create a new page
    let page = context.new_page().await?;
    println!("Created page with target: {}", page.target_id());

    // Navigate to a URL with default wait (Load event)
    println!("\nNavigating to example.com...");
    let response = page.goto("https://example.com").goto().await?;
    println!("Navigation complete! URL: {}", response.url());

    // Navigate with DomContentLoaded wait (faster)
    println!("\nNavigating to httpbin.org with DomContentLoaded wait...");
    let response = page
        .goto("https://httpbin.org/html")
        .wait_until(DocumentLoadState::DomContentLoaded)
        .timeout(Duration::from_secs(15))
        .goto()
        .await?;
    println!("Navigation complete! URL: {}", response.url());

    // Navigate with custom referer
    println!("\nNavigating with custom referer...");
    let response = page
        .goto("https://httpbin.org/headers")
        .referer("https://google.com")
        .wait_until(DocumentLoadState::Load)
        .goto()
        .await?;
    println!("Navigation complete! URL: {}", response.url());

    // Close the browser
    println!("\nClosing browser...");
    browser.close().await?;
    println!("Done!");

    Ok(())
}
