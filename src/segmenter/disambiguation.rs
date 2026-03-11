/// Disambiguation module: bidirectional verification.
/// Uses both forward and backward segmentation and picks the most consistent result.

use crate::segmenter::dict::Dictionary;
use crate::segmenter::dag::dag_segment;

/// Reverse segmentation: reverse the chars, segment, then reverse the result
fn segment_reversed(chars: &[char], dict: &Dictionary) -> Vec<(usize, usize)> {
    let n = chars.len();
    let reversed: Vec<char> = chars.iter().rev().cloned().collect();
    let rev_segs = dag_segment(&reversed, dict);
    
    // Convert reversed positions back to forward positions
    rev_segs.iter().map(|(s, e)| (n - e, n - s)).rev().collect()
}

/// Scoring function: fewer words is better (longer words preferred)
fn score(segs: &[(usize, usize)]) -> f64 {
    let total_words = segs.len() as f64;
    // Penalize single-char words
    let single_char_count = segs.iter().filter(|(s, e)| e - s == 1).count() as f64;
    -(total_words + single_char_count * 0.5)
}

/// Compare forward and reverse segmentations and return the better one
pub fn disambiguate(
    chars: &[char],
    forward: Vec<(usize, usize)>,
    dict: &Dictionary,
) -> Vec<(usize, usize)> {
    let reverse = segment_reversed(chars, dict);
    
    let fwd_score = score(&forward);
    let rev_score = score(&reverse);
    
    if fwd_score >= rev_score {
        forward
    } else {
        reverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segmenter::dict::Dictionary;

    #[test]
    fn test_disambiguation() {
        let mut dict = Dictionary::new();
        dict.load_from_str("南京市 100 ns\n长江大桥 80 ns\n南京 200 ns\n市长 150 n\n长江 300 ns\n大桥 100 n\n");
        let chars: Vec<char> = "南京市长江大桥".chars().collect();
        let forward = dag_segment(&chars, &dict);
        let result = disambiguate(&chars, forward, &dict);
        let words: Vec<String> = result.iter().map(|(s, e)| chars[*s..*e].iter().collect()).collect();
        println!("Disambiguated: {:?}", words);
    }
}
