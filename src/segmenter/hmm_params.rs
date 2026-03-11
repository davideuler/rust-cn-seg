// Auto-generated HMM parameters from jieba
// States: B=Begin, M=Middle, E=End, S=Single

pub const PROB_START_B: f64 = -0.26268660809250016;
pub const PROB_START_E: f64 = -3.14e+100;
pub const PROB_START_M: f64 = -3.14e+100;
pub const PROB_START_S: f64 = -1.4652633398537678;

// Transition: B->E, B->M
pub const PROB_TRANS_BE: f64 = -0.51082562376599;
pub const PROB_TRANS_BM: f64 = -0.916290731874155;
// Transition: E->B, E->S
pub const PROB_TRANS_EB: f64 = -0.5897149736854513;
pub const PROB_TRANS_ES: f64 = -0.8085250474669937;
// Transition: M->E, M->M
pub const PROB_TRANS_ME: f64 = -0.33344856811948514;
pub const PROB_TRANS_MM: f64 = -1.2603623820268226;
// Transition: S->B, S->S
pub const PROB_TRANS_SB: f64 = -0.7211965654669841;
pub const PROB_TRANS_SS: f64 = -0.6658631448798212;
