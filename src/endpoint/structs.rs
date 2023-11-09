use serde::{Deserialize, Serialize};
// use std::path::PathBuf;
use serde_json::Value;


#[derive(Deserialize)]
pub struct ContentQuery {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub addr: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub message: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct WebsiteData {
    websiteData: Vec<TextData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TextData {
    Title { r#type: String, text: String },
    Header { r#type: String, level: usize, text: String },
    Blockquote { r#type: String, text: String },
    CodeSnippet { r#type: String, code: String },
    Table { r#type: String, rows: Vec<TableRow> },
    Footer { r#type: String, text: String },
    Sidebar { r#type: String, text: String },
    Caption { r#type: String, text: String },
    Paragraph { r#type: String, text: String },
    BulletPoint { r#type: String, items: Vec<String> },
    NumberedPoint { r#type: String, items: Vec<String> },
    List { r#type: String, items: Vec<String> },
    Other(Value), // Catch-all for any other types not explicitly listed
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableRow {
    cells: Vec<String>,
}