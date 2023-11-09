use axum::extract::{Json, State};
use axum::http::StatusCode;
use time::Instant;
use crate::services::*;
use crate::endpoint::Result;
use crate::endpoint::structs::*;
use serde_json::Value;

// curl -X POST -H "Content-Type: application/json" -d '{"addr": "https://www.reddit.com/r/AskCulinary/comments/173g73f/is_lamb_a_gamey_meat/"}' http://0.0.0.0:8000/scrape_url
#[tracing::instrument(skip_all)]
pub async fn scrape_url(
    State(browser): State<BrowserService>,
    State(textr): State<TextExtractor>,
    Json(json): Json<GenerateRequest>,
) -> Result<(StatusCode, axum::Json<WebsiteData>)> {
    tracing::info!("Generating Video");
    let t0 = Instant::now();

    let base64_image = browser.take_screenshot_as_base64(&json.addr).await?;
    let content = textr.extract_text(&base64_image).await?;

    let website_data: WebsiteData = serde_json::from_str(&content).unwrap();
    // let cleaned_content = clean_json_value(&content)?;

    let ms = (Instant::now() - t0).whole_milliseconds();
    tracing::info!(target: "scrape_url", ms);

    Ok((StatusCode::OK,  Json(website_data)))
}
