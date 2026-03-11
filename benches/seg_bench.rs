use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rust_cn_seg::segmenter::{segment, SegMode};

const SHORT_TEXT: &str = "我来到北京清华大学";
const MEDIUM_TEXT: &str = "小明硕士毕业于中国科学院计算所，后来去了网易杭研大厦工作，每天研究生命科学和人工智能技术。";
const LONG_TEXT: &str = "中华人民共和国成立于1949年10月1日，首都北京。中国是世界上人口最多的国家，拥有14亿人口。\
    中国的经济在过去四十年间取得了举世瞩目的成就，成为世界第二大经济体。\
    北京是中国的政治、文化和国际交流中心。上海是中国最大的城市，也是重要的金融和商业中心。\
    深圳是中国最重要的科技创新城市之一，汇聚了华为、腾讯、大疆等众多知名企业。\
    中国的互联网产业蓬勃发展，阿里巴巴、百度、字节跳动等企业在全球具有重要影响力。";

fn bench_segment(c: &mut Criterion) {
    // Warm up the dictionary
    let _ = segment("预热", SegMode::Default);
    
    let mut group = c.benchmark_group("segment");
    
    group.throughput(Throughput::Bytes(SHORT_TEXT.len() as u64));
    group.bench_with_input(BenchmarkId::new("short", SHORT_TEXT.len()), &SHORT_TEXT, |b, text| {
        b.iter(|| segment(text, SegMode::Default))
    });
    
    group.throughput(Throughput::Bytes(MEDIUM_TEXT.len() as u64));
    group.bench_with_input(BenchmarkId::new("medium", MEDIUM_TEXT.len()), &MEDIUM_TEXT, |b, text| {
        b.iter(|| segment(text, SegMode::Default))
    });
    
    group.throughput(Throughput::Bytes(LONG_TEXT.len() as u64));
    group.bench_with_input(BenchmarkId::new("long", LONG_TEXT.len()), &LONG_TEXT, |b, text| {
        b.iter(|| segment(text, SegMode::Default))
    });
    
    group.finish();
}

criterion_group!(benches, bench_segment);
criterion_main!(benches);
