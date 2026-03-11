/// HMM + Viterbi for Out-of-Vocabulary (OOV) character segmentation.
/// States: B=Begin, M=Middle, E=End, S=Single
/// Uses parameters from jieba's original training data.

use std::collections::HashMap;
use once_cell::sync::Lazy;

// State indices
pub const B: usize = 0;
pub const M: usize = 1;
pub const E: usize = 2;
pub const S: usize = 3;
const NUM_STATES: usize = 4;

const MIN_FLOAT: f64 = -3.14e+100;

// Transition probabilities [from][to]
// Valid transitions: B->E, B->M, E->B, E->S, M->E, M->M, S->B, S->S
pub static TRANS: [[f64; NUM_STATES]; NUM_STATES] = [
    // to:  B            M            E            S
    [MIN_FLOAT, -0.916290731874155, -0.510825623765990, MIN_FLOAT], // from B
    [MIN_FLOAT, -1.2603623820268226, -0.33344856811948514, MIN_FLOAT], // from M
    [-0.5897149736854513, MIN_FLOAT, MIN_FLOAT, -0.8085250474669937], // from E
    [-0.7211965654669841, MIN_FLOAT, MIN_FLOAT, -0.6658631448798212], // from S
];

// Start probabilities
pub static START: [f64; NUM_STATES] = [
    -0.26268660809250016, // B
    MIN_FLOAT,            // M
    MIN_FLOAT,            // E
    -1.4652633398537678,  // S
];

// Emit probabilities loaded from JSON
pub static EMIT: Lazy<[HashMap<char, f64>; NUM_STATES]> = Lazy::new(|| {
    let json_str = include_str!("../data/prob_emit.json");
    let raw: serde_json::Value = serde_json::from_str(json_str).expect("Failed to parse prob_emit.json");
    
    let mut emit: [HashMap<char, f64>; NUM_STATES] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    
    let state_names = [("B", B), ("M", M), ("E", E), ("S", S)];
    for (name, idx) in &state_names {
        if let Some(state_data) = raw.get(name) {
            if let Some(map) = state_data.as_object() {
                for (char_str, prob) in map {
                    let c = char_str.chars().next().unwrap_or('\0');
                    let p = prob.as_f64().unwrap_or(MIN_FLOAT);
                    emit[*idx].insert(c, p);
                }
            }
        }
    }
    emit
});

/// Get emit probability for state and character
#[inline]
pub fn get_emit(state: usize, c: char) -> f64 {
    EMIT[state].get(&c).copied().unwrap_or(MIN_FLOAT)
}

/// Viterbi algorithm to find best state sequence
pub fn viterbi(chars: &[char]) -> Vec<usize> {
    let n = chars.len();
    if n == 0 {
        return vec![];
    }

    // viterbi[t][state] = log probability
    let mut vit = vec![[MIN_FLOAT; NUM_STATES]; n];
    let mut path = vec![[0usize; NUM_STATES]; n];

    // Initialize
    for s in 0..NUM_STATES {
        let emit = get_emit(s, chars[0]);
        if emit > MIN_FLOAT / 2.0 {
            vit[0][s] = START[s] + emit;
        }
    }

    // Forward
    for t in 1..n {
        for s in 0..NUM_STATES {
            let emit = get_emit(s, chars[t]);
            if emit <= MIN_FLOAT / 2.0 {
                continue;
            }
            let mut best_prob = MIN_FLOAT;
            let mut best_prev = 0;
            for prev in 0..NUM_STATES {
                if vit[t - 1][prev] <= MIN_FLOAT / 2.0 {
                    continue;
                }
                let trans = TRANS[prev][s];
                if trans <= MIN_FLOAT / 2.0 {
                    continue;
                }
                let p = vit[t - 1][prev] + trans + emit;
                if p > best_prob {
                    best_prob = p;
                    best_prev = prev;
                }
            }
            vit[t][s] = best_prob;
            path[t][s] = best_prev;
        }
    }

    // Backtrack
    let mut states = vec![0usize; n];
    // Find best final state
    let mut best_final = MIN_FLOAT;
    let mut best_s = S; // Default to S (single)
    for s in [E, S] {
        // Final state must be E or S
        if vit[n - 1][s] > best_final {
            best_final = vit[n - 1][s];
            best_s = s;
        }
    }
    
    // If no valid path found, default to all S
    if best_final <= MIN_FLOAT / 2.0 {
        return vec![S; n];
    }

    states[n - 1] = best_s;
    for t in (1..n).rev() {
        states[t - 1] = path[t][states[t]];
    }
    states
}

/// Convert HMM state sequence to word segments
/// Returns list of (start, end) pairs in char indices
pub fn hmm_cut(chars: &[char]) -> Vec<(usize, usize)> {
    if chars.is_empty() {
        return vec![];
    }
    
    let states = viterbi(chars);
    let mut result = Vec::new();
    let mut start = 0;
    
    for (i, &state) in states.iter().enumerate() {
        match state {
            S => {
                result.push((i, i + 1));
                start = i + 1;
            }
            E => {
                result.push((start, i + 1));
                start = i + 1;
            }
            B => {
                start = i;
            }
            _ => {} // M: continue
        }
    }
    
    // Handle trailing state
    if start < chars.len() {
        result.push((start, chars.len()));
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmm_basic() {
        // Force lazy init
        let _ = &*EMIT;
        
        let chars: Vec<char> = "杭研".chars().collect();
        let segs = hmm_cut(&chars);
        let words: Vec<String> = segs.iter().map(|(s, e)| chars[*s..*e].iter().collect()).collect();
        println!("HMM cut '杭研': {:?}", words);
        // This is OOV - should segment reasonably
    }
}
