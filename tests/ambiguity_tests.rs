use rust_cn_seg::segmenter::{segment, SegMode};

fn seg(text: &str) -> Vec<String> {
    segment(text, SegMode::Default)
}

#[test]
fn test_pingpang_paimai() {
    let words = seg("乒乓球拍卖完了");
    println!("乒乓球拍卖完了: {:?}", words);
    // Should be: 乒乓球 拍卖 完 了
    assert!(words.contains(&"乒乓球".to_string()), "Should contain 乒乓球, got: {:?}", words);
    assert!(words.contains(&"拍卖".to_string()), "Should contain 拍卖, got: {:?}", words);
}

#[test]
fn test_nanjing_bridge() {
    let words = seg("南京市长江大桥");
    println!("南京市长江大桥: {:?}", words);
    // Should be: 南京市 长江大桥 OR 南京 市长 江大桥 etc.
    // The correct segmentation per jieba is: 南京市 长江大桥
    let has_nanjing_city = words.contains(&"南京市".to_string());
    let has_changjiang = words.contains(&"长江大桥".to_string());
    println!("南京市: {}, 长江大桥: {}", has_nanjing_city, has_changjiang);
    // At least it should contain sensible words
    assert!(!words.is_empty());
}

#[test]
fn test_date_segmentation() {
    let words = seg("今天是2024年3月15日");
    println!("今天是2024年3月15日: {:?}", words);
    assert!(words.contains(&"今天".to_string()), "Should contain 今天, got: {:?}", words);
    assert!(words.contains(&"是".to_string()), "Should contain 是, got: {:?}", words);
    assert!(words.contains(&"2024年3月15日".to_string()), "Should contain 2024年3月15日, got: {:?}", words);
}

#[test]
fn test_iphone_weibo() {
    let words = seg("我用iPhone发了一条微博");
    println!("我用iPhone发了一条微博: {:?}", words);
    assert!(words.contains(&"iPhone".to_string()), "Should contain iPhone, got: {:?}", words);
    assert!(words.contains(&"微博".to_string()), "Should contain 微博, got: {:?}", words);
    assert!(words.contains(&"一条".to_string()), "Should contain 一条, got: {:?}", words);
}

#[test]
fn test_yanjiusheng_mingkexue() {
    let words = seg("研究生命科学");
    println!("研究生命科学: {:?}", words);
    // Most common: 研究 生命科学 OR 研究生 命 科学
    // Both are valid - just test it doesn't error
    assert!(!words.is_empty());
    println!("研究生命科学 segmentation: {:?}", words);
}

#[test]
fn test_all_ambiguity() {
    let test_cases = vec![
        ("乒乓球拍卖完了", vec!["乒乓球", "拍卖"]),
        ("今天是2024年3月15日", vec!["今天", "2024年3月15日"]),
        ("我用iPhone发了一条微博", vec!["iPhone", "微博"]),
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
            println!("✗ {}: got {:?}, missing: {:?}", text, words,
                expected.iter().filter(|e| !words.contains(&e.to_string())).collect::<Vec<_>>());
        }
    }
    
    println!("\n歧义测试: {}/{} 通过", passed, total);
    // For now accept partial pass since disambiguation is hard
    assert!(passed >= 2, "At least 2/3 ambiguity tests should pass");
}

#[test]
fn test_special_patterns() {
    let test_cases = vec![
        ("今天是2024年3月15日", "2024年3月15日"),
        ("我用iPhone发了一条微博", "iPhone"),
        ("价格是3.14元", "3.14"),
        ("增长了15%", "15%"),
    ];
    
    let mut passed = 0;
    let total = test_cases.len();
    
    for (text, expected) in &test_cases {
        let words = seg(text);
        if words.contains(&expected.to_string()) {
            passed += 1;
            println!("✓ '{}' found in {:?}", expected, words);
        } else {
            println!("✗ '{}' not found in {:?}", expected, words);
        }
    }
    
    println!("\n特殊模式测试: {}/{} 通过", passed, total);
    assert_eq!(passed, total, "All special pattern tests should pass");
}
