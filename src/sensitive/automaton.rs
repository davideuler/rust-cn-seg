/// Aho-Corasick automaton implementation from scratch.
/// Supports multi-pattern matching for sensitive word detection.

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Default)]
struct AcNode {
    children: HashMap<char, usize>,
    fail: usize,       // failure link
    output: Vec<String>, // words that end at this node
}

pub struct AhoCorasick {
    nodes: Vec<AcNode>,
    built: bool,
}

impl AhoCorasick {
    pub fn new() -> Self {
        let root = AcNode::default();
        AhoCorasick {
            nodes: vec![root],
            built: false,
        }
    }

    /// Add a pattern to the automaton (must call build() after adding all patterns)
    pub fn add_pattern(&mut self, pattern: &str) {
        if pattern.is_empty() {
            return;
        }
        let mut cur = 0;
        for c in pattern.chars() {
            let next = if let Some(&n) = self.nodes[cur].children.get(&c) {
                n
            } else {
                let new_id = self.nodes.len();
                self.nodes.push(AcNode::default());
                self.nodes[cur].children.insert(c, new_id);
                new_id
            };
            cur = next;
        }
        self.nodes[cur].output.push(pattern.to_string());
        self.built = false;
    }

    /// Build failure links using BFS
    pub fn build(&mut self) {
        let mut queue = VecDeque::new();
        
        // Initialize fail links for depth-1 nodes (collect first to avoid borrow issue)
        let root_children: Vec<usize> = self.nodes[0].children.values().cloned().collect();
        for child in root_children {
            self.nodes[child].fail = 0;
            queue.push_back(child);
        }
        
        while let Some(u) = queue.pop_front() {
            // Clone children to avoid borrow issues
            let children: Vec<(char, usize)> = self.nodes[u].children.iter().map(|(&c, &v)| (c, v)).collect();
            let u_fail = self.nodes[u].fail;
            
            for (c, v) in children {
                let mut fail = u_fail;
                loop {
                    if let Some(&next) = self.nodes[fail].children.get(&c) {
                        if next != v {
                            self.nodes[v].fail = next;
                            break;
                        }
                    }
                    if fail == 0 {
                        self.nodes[v].fail = 0;
                        break;
                    }
                    fail = self.nodes[fail].fail;
                }
                
                // Merge output from fail node
                let fail_node = self.nodes[v].fail;
                let fail_output: Vec<String> = self.nodes[fail_node].output.clone();
                self.nodes[v].output.extend(fail_output);
                
                queue.push_back(v);
            }
        }
        
        self.built = true;
    }

    /// Search for all pattern matches in text
    /// Returns list of (start_char, end_char, pattern)
    pub fn search(&self, text: &str) -> Vec<(usize, usize, String)> {
        assert!(self.built, "Must call build() before search()");
        
        let chars: Vec<char> = text.chars().collect();
        let mut results = Vec::new();
        let mut cur = 0;
        
        for (i, &c) in chars.iter().enumerate() {
            // Follow fail links until we find a transition or reach root
            loop {
                if let Some(&next) = self.nodes[cur].children.get(&c) {
                    cur = next;
                    break;
                }
                if cur == 0 {
                    break;
                }
                cur = self.nodes[cur].fail;
            }
            
            // Collect outputs at current node
            for pattern in &self.nodes[cur].output {
                let len = pattern.chars().count();
                let start = i + 1 - len;
                results.push((start, i + 1, pattern.clone()));
            }
        }
        
        results
    }

    pub fn pattern_count(&self) -> usize {
        self.nodes.iter().map(|n| n.output.len()).sum()
    }
}

/// High-level sensitive word detector
pub struct SensitiveDetector {
    ac: AhoCorasick,
    patterns: Vec<String>,
}

impl SensitiveDetector {
    pub fn new() -> Self {
        SensitiveDetector {
            ac: AhoCorasick::new(),
            patterns: Vec::new(),
        }
    }

    pub fn load_from_str(&mut self, data: &str) {
        for line in data.lines() {
            let word = line.trim();
            if !word.is_empty() && !word.starts_with('#') {
                self.add_word(word);
            }
        }
        self.ac.build();
    }

    pub fn add_word(&mut self, word: &str) {
        self.patterns.push(word.to_string());
        self.ac.add_pattern(word);
        // Note: must call rebuild() or build() after adding
    }

    pub fn rebuild(&mut self) {
        self.ac = AhoCorasick::new();
        for p in &self.patterns {
            self.ac.add_pattern(p);
        }
        self.ac.build();
    }

    /// Check if text contains any sensitive words
    pub fn contains_sensitive(&self, text: &str) -> bool {
        !self.ac.search(text).is_empty()
    }

    /// Get all sensitive words found in text
    pub fn find_all(&self, text: &str) -> Vec<SensitiveMatch> {
        self.ac.search(text).into_iter().map(|(start, end, word)| {
            SensitiveMatch { start, end, word }
        }).collect()
    }

    pub fn word_count(&self) -> usize {
        self.patterns.len()
    }
}

#[derive(Debug, Clone)]
pub struct SensitiveMatch {
    pub start: usize,
    pub end: usize,
    pub word: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick_basic() {
        let mut ac = AhoCorasick::new();
        ac.add_pattern("he");
        ac.add_pattern("she");
        ac.add_pattern("his");
        ac.add_pattern("hers");
        ac.build();
        
        let results = ac.search("ushers");
        let patterns: Vec<String> = results.iter().map(|(_, _, p)| p.clone()).collect();
        assert!(patterns.contains(&"she".to_string()));
        assert!(patterns.contains(&"he".to_string()));
        assert!(patterns.contains(&"hers".to_string()));
    }

    #[test]
    fn test_chinese_sensitive() {
        let mut detector = SensitiveDetector::new();
        detector.load_from_str("赌博\n毒品\n暴力");
        
        assert!(detector.contains_sensitive("这里有赌博活动"));
        assert!(!detector.contains_sensitive("正常的句子"));
        
        let matches = detector.find_all("这里有赌博和毒品");
        assert_eq!(matches.len(), 2);
    }
}
