# py-rs

汉字转拼音首字母命令行工具，Rust 实现，功能与 [muzimu/py](https://github.com/muzimu/py) 相同。

## 功能

读取标准输入中的汉字，输出拼音首字母，支持管道组合使用。

## 安装

```bash
cargo install --path .
```

## 用法

```
echo "<汉字文本>" | py [选项]
```

| 选项 | 简写 | 默认 | 说明 |
|------|------|------|------|
| `--lower` | `-l` | true | 首字母小写（默认） |
| `--upper` | `-u` | false | 首字母大写 |
| `--keep-non-han` | `-k` | true | 保留非汉字字符（数字、字母、符号等） |

> `-u` 与 `-l` 互斥，`-u` 显式指定时优先级更高。

## 示例

```bash
# 默认小写
echo "张三" | py
# 输出: zs

# 显式大写
echo "张三" | py -u
# 输出: ZS

# 含非汉字字符（默认保留）
echo "张三 2024" | py
# 输出: zs 2024

# 过滤非汉字字符
echo "张三 2024" | py -k=false
# 输出: zs
```

## 依赖

- [pinyin](https://crates.io/crates/pinyin) — 汉字转拼音
- [clap](https://crates.io/crates/clap) — 命令行参数解析
