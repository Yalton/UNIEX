use fantoccini::{ClientBuilder, Locator};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Clone)]
pub struct BrowserService {
    webdriver: String,
}
use base64::{engine::general_purpose, Engine as _};

const BODY: &str = r#"/html/body"#;

impl BrowserService {
    fn new(webdriver: &str) -> Self {
        let webdriver = webdriver.to_string();
        Self { webdriver }
    }

    pub async fn connect(webdriver: &str) -> anyhow::Result<Self> {
        // TODO: Test connection eg retrieve reddit main page.
        Ok(Self::new(webdriver))
    }

    pub async fn take_screenshot_as_base64(&self, url: &str) -> anyhow::Result<String> {
        let mut capabilities = serde_json::map::Map::new();
        let options = serde_json::json!({ "args": ["--headless"] });
        capabilities.insert("moz:firefoxOptions".to_string(), options);

        let client = ClientBuilder::native()
            .capabilities(capabilities)
            .connect(&self.webdriver)
            .await
            .expect("failed to connect to WebDriver");

        // Navigate to the URL
        client.goto(url).await?;

        let body = client.find(Locator::XPath(BODY)).await?;

        let screenshot = body.screenshot().await?;

        let mut file = File::create("screenshot.png").await?;
        file.write_all(&screenshot).await?;

        // Convert the screenshot to a base64 string
        let base64_screenshot = general_purpose::STANDARD.encode(&screenshot);

        // Ensure the browser is closed
        client.close().await?;

        Ok(base64_screenshot)
    }
}
