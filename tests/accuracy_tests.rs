use rust_cn_seg::segmenter::{segment, SegMode};

fn seg(text: &str) -> Vec<String> {
    segment(text, SegMode::Default)
}

#[test]
fn test_beijing_qinghua() {
    let words = seg("我来到北京清华大学");
    println!("我来到北京清华大学: {:?}", words);
    assert!(words.contains(&"北京".to_string()), "Should contain 北京, got: {:?}", words);
    assert!(words.contains(&"清华大学".to_string()), "Should contain 清华大学, got: {:?}", words);
    assert!(words.contains(&"来到".to_string()), "Should contain 来到, got: {:?}", words);
}

#[test]
fn test_xiaoming_keyan() {
    let words = seg("小明硕士毕业于中国科学院计算所");
    println!("小明硕士毕业于中国科学院计算所: {:?}", words);
    assert!(words.contains(&"小明".to_string()), "Should contain 小明, got: {:?}", words);
    assert!(words.contains(&"硕士".to_string()), "Should contain 硕士, got: {:?}", words);
    assert!(words.contains(&"毕业".to_string()), "Should contain 毕业, got: {:?}", words);
    assert!(words.contains(&"中国科学院".to_string()), "Should contain 中国科学院, got: {:?}", words);
    assert!(words.contains(&"计算所".to_string()), "Should contain 计算所, got: {:?}", words);
}

#[test]
fn test_wangyi_hangyan() {
    let words = seg("他来到了网易杭研大厦");
    println!("他来到了网易杭研大厦: {:?}", words);
    assert!(words.contains(&"网易".to_string()), "Should contain 网易, got: {:?}", words);
    assert!(words.contains(&"大厦".to_string()), "Should contain 大厦, got: {:?}", words);
    assert!(words.contains(&"来到".to_string()), "Should contain 来到, got: {:?}", words);
}

#[test]
fn test_all_basic() {
    let test_cases = vec![
        ("我来到北京清华大学", vec!["我", "来到", "北京", "清华大学"]),
        ("小明硕士毕业于中国科学院计算所", vec!["小明", "硕士", "毕业", "于", "中国科学院", "计算所"]),
        ("他来到了网易杭研大厦", vec!["他", "来到", "了", "网易", "杭研", "大厦"]),
    ];
    
    let mut passed = 0;
    let total = test_cases.len();
    
    for (text, expected) in &test_cases {
        let words = seg(text);
        let all_found = expected.iter().all(|e| words.contains(&e.to_string()));
        if all_found {
            passed += 1;
            println!("✓ {}: {:?}", text, words);
        } else {
            println!("✗ {}: got {:?}, expected all of {:?}", text, words, expected);
        }
    }
    
    println!("\n基础测试: {}/{} 通过", passed, total);
    assert_eq!(passed, total, "All basic tests should pass");
}
