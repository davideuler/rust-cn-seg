use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber;
use tower_http::services::ServeDir;

mod segmenter;
mod sensitive;
mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Pre-initialize the global dictionary (avoids slow first request)
    info!("Loading dictionary...");
    let dict_size = {
        let dict = segmenter::GLOBAL_DICT.read().unwrap();
        dict.word_count()
    };
    info!("Dictionary loaded: {} words", dict_size);
    
    // Pre-initialize HMM emit table
    let _ = &*segmenter::hmm::EMIT;
    info!("HMM parameters loaded");
    
    // Pre-initialize sensitive detector
    let sensitive_size = {
        let detector = sensitive::GLOBAL_SENSITIVE.read().unwrap();
        detector.word_count()
    };
    info!("Sensitive detector loaded: {} patterns", sensitive_size);
    
    let app = api::create_router()
        .nest_service("/", ServeDir::new("static"));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    info!("Starting server on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
