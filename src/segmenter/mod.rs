pub mod dict;
pub mod dag;
pub mod hmm;
pub mod pattern;
pub mod disambiguation;
mod hmm_params;

use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::segmenter::dict::Dictionary;
use crate::segmenter::dag::dag_segment;
use crate::segmenter::hmm::hmm_cut;
use crate::segmenter::pattern::{split_into_chunks, Chunk};
use crate::segmenter::disambiguation::disambiguate;

const DICT_DATA: &str = include_str!("../data/dict.txt");

pub static GLOBAL_DICT: Lazy<Arc<RwLock<Dictionary>>> = Lazy::new(|| {
    let mut dict = Dictionary::new();
    dict.load_from_str(DICT_DATA);
    
    // Add important words that might be missing
    let extra_words = [
        ("网易", 1000, "nz"),
        ("杭研", 500, "nz"),
        ("大厦", 800, "n"),
        ("计算所", 600, "nz"),
        ("中国科学院", 1000, "nt"),
        ("小明", 500, "nr"),  // common name
        ("微博", 1000, "nz"), // Weibo social platform
    ];
    for (word, freq, pos) in &extra_words {
        if !dict.is_word(word) {
            dict.add_word(word, *freq, pos.to_string());
        }
    }
    dict.compute_log_probs();
    
    Arc::new(RwLock::new(dict))
});

#[derive(Debug, Clone, PartialEq)]
pub enum SegMode {
    Default,  // Standard segmentation
    Search,   // Search mode: further split long words
    Fine,     // Fine-grained: maximize splits
}

/// Main segmentation function
pub fn segment(text: &str, mode: SegMode) -> Vec<String> {
    let dict = GLOBAL_DICT.read().unwrap();
    segment_with_dict(text, mode, &dict)
}

pub fn segment_with_dict(text: &str, mode: SegMode, dict: &Dictionary) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }

    let chunks = split_into_chunks(text);
    let mut result = Vec::new();

    for chunk in chunks {
        match chunk {
            Chunk::Pattern(pat) => {
                result.push(pat.token);
            }
            Chunk::Chinese(text_chunk, _) => {
                let words = segment_chinese(&text_chunk, &mode, dict);
                result.extend(words);
            }
        }
    }

    result
}

/// Segment a pure Chinese text chunk
fn segment_chinese(text: &str, mode: &SegMode, dict: &Dictionary) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }

    let chars: Vec<char> = text.chars().collect();
    let mut result = Vec::new();
    
    // Find continuous CJK blocks and handle non-CJK chars (punctuation etc.)
    let mut start = 0;
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        if is_cjk(c) || is_cjk_compatible(c) {
            i += 1;
        } else {
            // Flush CJK block
            if start < i {
                let cjk_chars = &chars[start..i];
                let words = segment_cjk_block(cjk_chars, mode, dict);
                result.extend(words);
            }
            // Output non-CJK char as-is (punctuation, etc.)
            if !c.is_whitespace() {
                result.push(c.to_string());
            }
            i += 1;
            start = i;
        }
    }
    
    // Flush remaining CJK
    if start < chars.len() {
        let cjk_chars = &chars[start..];
        let words = segment_cjk_block(cjk_chars, mode, dict);
        result.extend(words);
    }
    
    result
}

/// Segment a block of CJK characters
fn segment_cjk_block(chars: &[char], mode: &SegMode, dict: &Dictionary) -> Vec<String> {
    if chars.is_empty() {
        return vec![];
    }

    // DAG segmentation
    let forward_segs = dag_segment(chars, dict);
    
    // Disambiguate using bidirectional verification
    let segs = disambiguate(chars, forward_segs, dict);
    
    let mut result = Vec::new();
    
    for (start, end) in &segs {
        let word: String = chars[*start..*end].iter().collect();
        
        // Check if this is an OOV that needs HMM
        if !dict.is_word(&word) && word.chars().count() > 1 {
            // Apply HMM to this OOV segment
            let word_chars: Vec<char> = word.chars().collect();
            let hmm_segs = hmm_cut(&word_chars);
            for (hs, he) in hmm_segs {
                let w: String = word_chars[hs..he].iter().collect();
                result.push(w);
            }
        } else {
            match mode {
                SegMode::Search => {
                    // In search mode, also split long words into sub-words
                    let sub_words = search_mode_split(&word, dict);
                    result.extend(sub_words);
                }
                _ => {
                    result.push(word);
                }
            }
        }
    }
    
    result
}

/// Search mode: split long words into shorter sub-words for better search coverage
fn search_mode_split(word: &str, dict: &Dictionary) -> Vec<String> {
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    
    if n <= 2 {
        return vec![word.to_string()];
    }
    
    let mut result = vec![word.to_string()];
    
    // Try bigrams first
    if n >= 4 {
        for i in 0..n - 1 {
            let bigram: String = chars[i..i + 2].iter().collect();
            if dict.is_word(&bigram) {
                result.push(bigram);
            }
        }
    }
    
    result
}

/// Check if char is CJK
#[inline]
fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x4E00..=0x9FFF   // CJK Unified Ideographs
        | 0x3400..=0x4DBF // CJK Extension A
        | 0x20000..=0x2A6DF // CJK Extension B
        | 0x2A700..=0x2B73F // CJK Extension C
        | 0x2B740..=0x2B81F // CJK Extension D
        | 0xF900..=0xFAFF  // CJK Compatibility Ideographs
    )
}

#[inline]
fn is_cjk_compatible(c: char) -> bool {
    // Include common Chinese punctuation and symbols that should be grouped with Chinese text
    matches!(c as u32,
        0x3000..=0x303F   // CJK Symbols and Punctuation
        | 0xFF00..=0xFFEF // Halfwidth and Fullwidth Forms  
    )
}

/// Add a word to the global dictionary
pub fn add_word(word: &str, freq: u32, pos: &str) {
    let mut dict = GLOBAL_DICT.write().unwrap();
    dict.add_word(word, freq, pos.to_string());
    dict.compute_log_probs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_init() {
        let dict = GLOBAL_DICT.read().unwrap();
        assert!(dict.word_count() > 1000, "Dictionary should have many words");
    }
    
    #[test]
    fn test_basic_segment() {
        let words = segment("我来到北京", SegMode::Default);
        println!("我来到北京: {:?}", words);
        assert!(!words.is_empty());
    }
}
