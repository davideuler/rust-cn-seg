/// Dictionary using a hash map for O(1) average lookup.
/// Stores word -> (frequency, part-of-speech).
/// Also builds a prefix set for efficient DAG construction.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WordEntry {
    pub freq: u32,
    pub pos: String,
    pub log_prob: f64,
}

pub struct Dictionary {
    pub words: HashMap<String, WordEntry>,
    pub prefixes: HashMap<String, bool>, // true = full word, false = prefix only
    pub total: u64,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            words: HashMap::new(),
            prefixes: HashMap::new(),
            total: 0,
        }
    }

    pub fn load_from_str(&mut self, data: &str) {
        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.is_empty() {
                continue;
            }
            let word = parts[0];
            if word.is_empty() {
                continue;
            }
            let freq: u32 = if parts.len() > 1 {
                parts[1].parse().unwrap_or(1)
            } else {
                1
            };
            let pos = if parts.len() > 2 {
                parts[2].to_string()
            } else {
                "n".to_string()
            };
            self.add_word(word, freq, pos);
        }
        self.compute_log_probs();
    }

    pub fn add_word(&mut self, word: &str, freq: u32, pos: String) {
        // Add all prefixes to prefix set
        let chars: Vec<char> = word.chars().collect();
        let mut prefix = String::new();
        for (i, c) in chars.iter().enumerate() {
            prefix.push(*c);
            let is_full = i == chars.len() - 1;
            let entry = self.prefixes.entry(prefix.clone()).or_insert(false);
            if is_full {
                *entry = true;
            }
        }

        let old_freq = self.words.get(word).map(|e| e.freq).unwrap_or(0);
        self.total = self.total.saturating_sub(old_freq as u64);
        self.total += freq as u64;

        self.words.insert(word.to_string(), WordEntry {
            freq,
            pos,
            log_prob: 0.0, // will be computed
        });
    }

    pub fn compute_log_probs(&mut self) {
        let total = if self.total == 0 { 1 } else { self.total };
        for entry in self.words.values_mut() {
            if entry.freq > 0 {
                entry.log_prob = (entry.freq as f64).ln() - (total as f64).ln();
            } else {
                entry.log_prob = -20.0; // very low probability
            }
        }
    }

    #[inline]
    pub fn get_word(&self, word: &str) -> Option<&WordEntry> {
        self.words.get(word)
    }

    #[inline]
    pub fn is_prefix(&self, prefix: &str) -> bool {
        self.prefixes.contains_key(prefix)
    }

    #[inline]
    pub fn is_word(&self, word: &str) -> bool {
        self.prefixes.get(word).copied().unwrap_or(false)
    }

    pub fn get_log_prob(&self, word: &str) -> f64 {
        match self.get_word(word) {
            Some(e) => e.log_prob,
            None => {
                // Unknown word penalty - scale by length
                let len = word.chars().count();
                -20.0 * len as f64
            }
        }
    }

    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict_basic() {
        let mut dict = Dictionary::new();
        dict.load_from_str("北京 100 ns\n清华大学 50 nt\n");
        assert!(dict.is_word("北京"));
        assert!(dict.is_word("清华大学"));
        assert!(dict.is_prefix("清华"));
        assert!(!dict.is_word("清华"));
    }
}
