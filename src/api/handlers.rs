use axum::{extract::Json, http::StatusCode};
use std::time::Instant;
use crate::api::models::*;
use crate::segmenter::{segment, add_word, SegMode, GLOBAL_DICT};
use crate::sensitive::{detect, add_sensitive_word, GLOBAL_SENSITIVE};

fn parse_mode(mode: &str) -> SegMode {
    match mode {
        "search" => SegMode::Search,
        "fine" => SegMode::Fine,
        _ => SegMode::Default,
    }
}

pub async fn segment_handler(
    Json(req): Json<SegmentRequest>,
) -> Result<Json<SegmentResponse>, StatusCode> {
    let start = Instant::now();
    let mode = parse_mode(&req.mode);
    let words = segment(&req.text, mode);
    let count = words.len();
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    Ok(Json(SegmentResponse {
        words,
        count,
        elapsed_ms,
    }))
}

pub async fn sensitive_handler(
    Json(req): Json<SensitiveRequest>,
) -> Result<Json<SensitiveResponse>, StatusCode> {
    let start = Instant::now();
    let words = detect(&req.text);
    let found = !words.is_empty();
    let count = words.len();
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    Ok(Json(SensitiveResponse {
        found,
        words,
        count,
        elapsed_ms,
    }))
}

pub async fn analyze_handler(
    Json(req): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, StatusCode> {
    let start = Instant::now();
    let mode = parse_mode(&req.mode);
    let words = segment(&req.text, mode);
    let sensitive_words = detect(&req.text);
    let word_count = words.len();
    let sensitive_found = !sensitive_words.is_empty();
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    
    Ok(Json(AnalyzeResponse {
        words,
        word_count,
        sensitive_found,
        sensitive_words,
        elapsed_ms,
    }))
}

pub async fn health_handler() -> Json<HealthResponse> {
    let dict_size = {
        let dict = GLOBAL_DICT.read().unwrap();
        dict.word_count()
    };
    let sensitive_size = {
        let detector = GLOBAL_SENSITIVE.read().unwrap();
        detector.word_count()
    };
    
    Json(HealthResponse {
        status: "ok".to_string(),
        dict_size,
        sensitive_size,
    })
}

pub async fn add_word_handler(
    Json(req): Json<AddWordRequest>,
) -> Result<Json<AddWordResponse>, StatusCode> {
    add_word(&req.word, req.freq, &req.pos);
    Ok(Json(AddWordResponse {
        success: true,
        word: req.word,
    }))
}
