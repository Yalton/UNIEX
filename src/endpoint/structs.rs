use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
