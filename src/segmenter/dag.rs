/// DAG (Directed Acyclic Graph) + Dynamic Programming for Chinese segmentation.
/// For each position i in the input, find all words starting at i.
/// Then use Viterbi-like DP to find the maximum probability path.

use crate::segmenter::dict::Dictionary;

/// Build DAG: dag[i] = list of end positions j such that text[i..j] is a word
pub fn build_dag(chars: &[char], dict: &Dictionary) -> Vec<Vec<usize>> {
    let n = chars.len();
    let mut dag = vec![vec![]; n];

    for i in 0..n {
        let mut word = String::new();
        for j in i..n {
            word.push(chars[j]);
            if !dict.is_prefix(&word) {
                break;
            }
            if dict.is_word(&word) {
                dag[i].push(j + 1);
            }
        }
        // Always allow single char
        if dag[i].is_empty() {
            dag[i].push(i + 1);
        }
    }
    dag
}

/// DP over DAG to find best segmentation path
/// Returns a list of (start, end) pairs
pub fn dag_segment(chars: &[char], dict: &Dictionary) -> Vec<(usize, usize)> {
    let n = chars.len();
    if n == 0 {
        return vec![];
    }

    let dag = build_dag(chars, dict);

    // dp[i] = (best log prob ending at i, previous position)
    let mut dp = vec![(f64::NEG_INFINITY, 0usize); n + 1];
    dp[0] = (0.0, 0);

    for i in 0..n {
        if dp[i].0 == f64::NEG_INFINITY && i > 0 {
            continue;
        }
        for &j in &dag[i] {
            let word: String = chars[i..j].iter().collect();
            let log_prob = dict.get_log_prob(&word);
            let new_prob = dp[i].0 + log_prob;
            if new_prob > dp[j].0 {
                dp[j] = (new_prob, i);
            }
        }
    }

    // Backtrack
    let mut result = Vec::new();
    let mut pos = n;
    while pos > 0 {
        let prev = dp[pos].1;
        result.push((prev, pos));
        pos = prev;
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segmenter::dict::Dictionary;

    #[test]
    fn test_dag_basic() {
        let mut dict = Dictionary::new();
        dict.load_from_str("北京 100 ns\n来到 80 v\n");
        let chars: Vec<char> = "来到北京".chars().collect();
        let segs = dag_segment(&chars, &dict);
        let words: Vec<String> = segs.iter().map(|(s, e)| chars[*s..*e].iter().collect()).collect();
        println!("Segmented: {:?}", words);
        assert!(words.contains(&"来到".to_string()));
        assert!(words.contains(&"北京".to_string()));
    }
}
