pub mod automaton;

use once_cell::sync::Lazy;
use std::sync::RwLock;
use crate::sensitive::automaton::SensitiveDetector;

const SENSITIVE_DATA: &str = include_str!("../data/sensitive.txt");

pub static GLOBAL_SENSITIVE: Lazy<RwLock<SensitiveDetector>> = Lazy::new(|| {
    let mut detector = SensitiveDetector::new();
    detector.load_from_str(SENSITIVE_DATA);
    RwLock::new(detector)
});

pub fn detect(text: &str) -> Vec<String> {
    let detector = GLOBAL_SENSITIVE.read().unwrap();
    detector.find_all(text).into_iter().map(|m| m.word).collect()
}

pub fn add_sensitive_word(word: &str) {
    let mut detector = GLOBAL_SENSITIVE.write().unwrap();
    detector.add_word(word);
    detector.rebuild();
}
