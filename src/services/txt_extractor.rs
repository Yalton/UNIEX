use std::sync::Arc;
use reqwest::{Client, ClientBuilder};
use time::Instant;


#[derive(Debug, Clone)]
pub struct TextExtractor {
    client: Client,
    auth: Arc<String>,
}

impl TextExtractor {
    pub fn new(auth: &str) -> anyhow::Result<Self> {
        let client = ClientBuilder::default().build()?;
        let auth = Arc::new(auth.to_owned());
        Ok(Self { client, auth })
    }

    #[tracing::instrument(skip_all)]
    pub async fn extract_text(&self, base64_image: &str) -> anyhow::Result<String> {
        let t0 = Instant::now();

        let addr = "https://api.openai.com/v1/chat/completions";
        let builder = self.client.post(addr);

        const AUTH: &str = "Authorization";
        let auth_header = format!("Bearer {}", self.auth.as_ref());

        let body = serde_json::json!({
            "model": "gpt-4-vision-preview",
            "messages": [
                {
                  "role": "user",
                  "content": [
                    {
                      "type": "text",
                                "text": "Extract all of the text from this screenshot, format the text extractions using proper markdown. Keep your response brief and do not include anything that is not the extracted text formatted as markdown. Use Markdown syntax characters where appropriate such that the markdown would resemble the original page as closely as the image represents it. Do not include ```markdown or ```"
                    },
                    {
                      "type": "image_url",
                      "image_url": {
                        "url": format!("data:image/jpeg;base64,{}", base64_image)
                    }
                    }
                  ]
                }
              ],
              "max_tokens": 3072
        });

        let req = builder
            .header("Content-Type", "application/json")
            .header(AUTH, auth_header)
            .json(&body);

        let resp = req.send().await?.text().await?;

        println!("{}", resp);

        let parsed: serde_json::Value = serde_json::from_str(&resp)?;

        // println!()
        let content = parsed["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default();

        let ms = (Instant::now() - t0).whole_milliseconds();
        tracing::info!(target: "openai", ms);
        Ok(content.to_string())
    }
}
