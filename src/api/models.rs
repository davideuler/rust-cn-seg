use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SegmentRequest {
    pub text: String,
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String {
    "default".to_string()
}

#[derive(Debug, Serialize)]
pub struct SegmentResponse {
    pub words: Vec<String>,
    pub count: usize,
    pub elapsed_ms: f64,
}

#[derive(Debug, Deserialize)]
pub struct SensitiveRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct SensitiveResponse {
    pub found: bool,
    pub words: Vec<String>,
    pub count: usize,
    pub elapsed_ms: f64,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub text: String,
    #[serde(default = "default_mode")]
    pub mode: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub words: Vec<String>,
    pub word_count: usize,
    pub sensitive_found: bool,
    pub sensitive_words: Vec<String>,
    pub elapsed_ms: f64,
}

#[derive(Debug, Deserialize)]
pub struct AddWordRequest {
    pub word: String,
    #[serde(default = "default_freq")]
    pub freq: u32,
    #[serde(default = "default_pos")]
    pub pos: String,
}

fn default_freq() -> u32 { 1000 }
fn default_pos() -> String { "n".to_string() }

#[derive(Debug, Serialize)]
pub struct AddWordResponse {
    pub success: bool,
    pub word: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub dict_size: usize,
    pub sensitive_size: usize,
}
