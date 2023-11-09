use reqwest::{Client, ClientBuilder};
use std::sync::Arc;
use time::Instant;

#[derive(Debug, Clone)]
pub struct TextExtractor {
    client: Client,
    auth: Arc<String>,
}

const JSON_STR: &str = r#"
{
  "websiteData": [
    {
      "textData": [
        {
          "type": "title",
          "text": "Example Website Title"
        },
        {
          "type": "header",
          "level": 1,
          "text": "Main Heading Text"
        },
        {
          "type": "header",
          "level": 2,
          "text": "Subheading Text"
        },
        {
          "type": "blockquote",
          "text": "This is a highlighted blockquote from the content."
        },
        {
          "type": "codeSnippet",
          "code": "console.log('Hello World');"
        },
        {
          "type": "table",
          "rows": [
            {
              "cells": ["Cell 1", "Cell 2"]
            },
            {
              "cells": ["Cell 3", "Cell 4"]
            }
          ]
        },
        {
          "type": "footer",
          "text": "Copyright information and other footer details."
        },
        {
          "type": "sidebar",
          "text": "Supplementary sidebar information or links."
        },
        {
          "type": "caption",
          "text": "This is a caption for a figure or section."
        },
        {
          "type": "paragraph",
          "text": "Some introductory paragraph text."
        },
        {
          "type": "bulletPoint",
          "items": [
            "Bullet item 1",
            "Bullet item 2"
          ]
        },
        {
          "type": "numberedPoint",
          "items": [
            "Numbered item 1",
            "Numbered item 2"
          ]
        },
        {
          "type": "list",
          "items": [
            "List item 1",
            "List item 2",
            "List item 3"
          ]
        }
      ]
    }
  ]
}
"#;

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

        let prompt = format!(
            r#"""Extract all text from the provided image. Organize the extracted text into a structured JSON format that aligns with the {JSON_STR} variable. Keep your response brief and do not include anything that is not the extracted text formatted as json The response should only include the JSON structure with the extracted text, without any formatting characters. Ensure that the JSON output accurately reflects the layout and content elements found in the image.Do not include ```json or ```"""#
        );
        let body = serde_json::json!({
            "model": "gpt-4-vision-preview",
            "messages": [
                {
                  "role": "user",
                  "content": [
                    {
                      "type": "text",
                      "text": prompt
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
