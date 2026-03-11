# rust-cn-seg

高性能中文分词服务，从零用 Rust 实现，无依赖任何第三方分词库。

## 特性

- **双数组 Trie 词典**：基于 HashMap 的高效词典，O(1) 平均查找
- **DAG + 动态规划**：构建有向无环图，最大概率路径求解
- **HMM + Viterbi**：基于 BMES 状态的未登录词处理
- **Aho-Corasick 自动机**：从零实现，用于敏感词检测
- **特殊模式**：日期、数字、英文、英数混合
- **歧义消解**：双向验证
- **REST API + Web UI**

## 性能（Release Build）

| 文本长度 | 耗时 | 吞吐量 |
|---------|------|--------|
| 短文本 (~27字节) | ~7μs | ~3.6 MiB/s |
| 中文本 (~135字节) | ~40μs | ~3.2 MiB/s |
| 长文本 (~585字节) | ~215μs | ~2.6 MiB/s |

## 准确性

```
基础测试: 4/4 通过
歧义测试: 7/7 通过
```

示例：
- `南京市长江大桥` → `["南京市", "长江大桥"]` ✓
- `乒乓球拍卖完了` → `["乒乓球", "拍卖", "完", "了"]` ✓
- `今天是2024年3月15日` → `["今天", "是", "2024年3月15日"]` ✓
- `我用iPhone发了一条微博` → `["我", "用", "iPhone", "发", "了", "一条", "微博"]` ✓

## 安装

```bash
cargo build --release
```

## 运行

```bash
./target/release/rust-cn-seg
# 服务启动在 http://localhost:3001
```

![Segmentation Service Webpage](static/segmentation.png)


## API

### 分词
```bash
curl -X POST http://localhost:3001/api/segment \
  -H "Content-Type: application/json" \
  -d '{"text": "我来到北京清华大学", "mode": "default"}'
```

### 敏感词检测
```bash
curl -X POST http://localhost:3001/api/sensitive \
  -H "Content-Type: application/json" \
  -d '{"text": "这里有非法活动"}'
```

### 综合分析
```bash
curl -X POST http://localhost:3001/api/analyze \
  -H "Content-Type: application/json" \
  -d '{"text": "我来到北京清华大学"}'
```

### 添加词汇
```bash
curl -X POST http://localhost:3001/api/dict/add \
  -H "Content-Type: application/json" \
  -d '{"word": "新词", "freq": 1000, "pos": "n"}'
```

### 健康检查
```bash
curl http://localhost:3001/api/health
```

## 分词模式

- `default`：标准模式
- `search`：搜索模式（长词进一步切分）
- `fine`：精细模式

## 算法说明

### 词典
使用 jieba 词典数据（~35万词条），自建 HashMap 索引 + 前缀树支持。

### DAG + DP
对每个位置查找所有可能以该位置开始的词，构建 DAG，然后用动态规划找最大对数概率路径。

### HMM
对未登录词（OOV）使用 HMM 的 Viterbi 算法，状态为 BMES（首字、中字、末字、单字），参数来自 jieba 原始训练数据。

### Aho-Corasick
从零实现的 AC 自动机，支持多模式字符串匹配，用于敏感词检测。

## 测试

```bash
cargo test
```

## Benchmark

```bash
cargo bench
```

## 许可证

MIT
