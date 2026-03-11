/// Pattern matching for special tokens: numbers, dates, English words, mixed alphanumeric.
/// These patterns take priority over dictionary lookup.

use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub start: usize, // byte offset
    pub end: usize,   // byte offset
    pub token: String,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Date,
    Time,
    Number,
    English,
    Mixed,  // e.g., iPhone16
}

// Patterns ordered by priority (more specific first)
static PATTERNS: Lazy<Vec<(Regex, TokenKind)>> = Lazy::new(|| {
    vec![
        // Date: 2024年3月15日, 2024-03-15
        (Regex::new(r"\d{2,4}年\d{1,2}月\d{1,2}[日号]?").unwrap(), TokenKind::Date),
        // Time: 12:30:00
        (Regex::new(r"\d{1,2}:\d{2}(:\d{2})?").unwrap(), TokenKind::Time),
        // Decimal/percent: 3.14, 15%
        (Regex::new(r"\d+\.\d+%?|\d+%").unwrap(), TokenKind::Number),
        // Mixed alphanumeric: iPhone16, MP4, 4G, 5G
        (Regex::new(r"[A-Za-z]+\d+[A-Za-z]*|[A-Za-z]*\d+[A-Za-z]+").unwrap(), TokenKind::Mixed),
        // English words (allow apostrophes, hyphens)
        (Regex::new(r"[A-Za-z][A-Za-z'\-]*[A-Za-z]|[A-Za-z]").unwrap(), TokenKind::English),
        // Pure integers
        (Regex::new(r"\d+").unwrap(), TokenKind::Number),
    ]
});

/// Find all pattern matches in text, returning non-overlapping matches
pub fn find_patterns(text: &str) -> Vec<PatternMatch> {
    let mut matches: Vec<PatternMatch> = Vec::new();
    let mut covered: Vec<bool> = vec![false; text.len()];

    for (pattern, kind) in PATTERNS.iter() {
        for mat in pattern.find_iter(text) {
            let start = mat.start();
            let end = mat.end();
            
            // Check if this range overlaps with already covered bytes
            if covered[start..end].iter().any(|&c| c) {
                continue;
            }
            
            // Mark as covered
            for b in start..end {
                covered[b] = true;
            }
            
            matches.push(PatternMatch {
                start,
                end,
                token: mat.as_str().to_string(),
                kind: kind.clone(),
            });
        }
    }
    
    // Sort by position
    matches.sort_by_key(|m| m.start);
    matches
}

/// Split text into segments: pattern matches and non-pattern chunks
#[derive(Debug)]
pub enum Chunk {
    Pattern(PatternMatch),
    Chinese(String, usize), // text, byte start
}

pub fn split_into_chunks(text: &str) -> Vec<Chunk> {
    let patterns = find_patterns(text);
    let mut chunks = Vec::new();
    let mut pos = 0; // byte position
    
    for pat in &patterns {
        if pos < pat.start {
            // Chinese/CJK chunk
            let chinese = &text[pos..pat.start];
            if !chinese.is_empty() {
                chunks.push(Chunk::Chinese(chinese.to_string(), pos));
            }
        }
        chunks.push(Chunk::Pattern(pat.clone()));
        pos = pat.end;
    }
    
    if pos < text.len() {
        let remaining = &text[pos..];
        if !remaining.is_empty() {
            chunks.push(Chunk::Chinese(remaining.to_string(), pos));
        }
    }
    
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_pattern() {
        let text = "今天是2024年3月15日";
        let chunks = split_into_chunks(text);
        let has_date = chunks.iter().any(|c| match c {
            Chunk::Pattern(p) => p.kind == TokenKind::Date && p.token == "2024年3月15日",
            _ => false,
        });
        assert!(has_date, "Should find date pattern");
    }

    #[test]
    fn test_english_pattern() {
        let text = "我用iPhone发了";
        let chunks = split_into_chunks(text);
        let has_eng = chunks.iter().any(|c| match c {
            Chunk::Pattern(p) => p.token == "iPhone",
            _ => false,
        });
        assert!(has_eng, "Should find iPhone");
    }
}
