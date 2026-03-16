use clap::Parser;
use pinyin::ToPinyin;
use std::io::{self, BufRead};

/// 汉字转拼音首字母命令行工具
///
/// 读取标准输入中的汉字，输出拼音首字母，支持管道组合使用。
///
/// 示例:
///   echo "张三" | py
///   echo "张三" | py -u
///   echo "张三 2024" | py -k=false
#[derive(Parser, Debug)]
#[command(name = "py", version, about, long_about = None)]
struct Args {
    /// 首字母小写（默认）
    #[arg(short = 'l', long = "lower", default_value_t = true, overrides_with = "upper")]
    lower: bool,

    /// 首字母大写
    #[arg(short = 'u', long = "upper", default_value_t = false)]
    upper: bool,

    /// 保留非汉字字符（数字、字母、符号等）
    #[arg(short = 'k', long = "keep-non-han", default_value_t = true)]
    keep_non_han: bool,
}

fn process_line(line: &str, uppercase: bool, keep_non_han: bool) -> String {
    let mut result = String::new();

    for ch in line.chars() {
        if let Some(pinyin_iter) = ch.to_pinyin() {
            // 是汉字，取拼音首字母
            let py = pinyin_iter.plain();
            if let Some(first) = py.chars().next() {
                if uppercase {
                    result.push(first.to_uppercase().next().unwrap_or(first));
                } else {
                    result.push(first.to_lowercase().next().unwrap_or(first));
                }
            }
        } else {
            // 非汉字字符
            if keep_non_han {
                result.push(ch);
            }
        }
    }

    result
}

fn main() {
    let args = Args::parse();

    // -u 显式指定时优先级更高
    let uppercase = args.upper;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                let output = process_line(&l, uppercase, args.keep_non_han);
                println!("{}", output);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lowercase() {
        assert_eq!(process_line("张三", false, true), "zs");
    }

    #[test]
    fn test_basic_uppercase() {
        assert_eq!(process_line("张三", true, true), "ZS");
    }

    #[test]
    fn test_keep_non_han() {
        assert_eq!(process_line("张三 2024", false, true), "zs 2024");
    }

    #[test]
    fn test_filter_non_han() {
        assert_eq!(process_line("张三 2024", false, false), "zs");
    }

    #[test]
    fn test_mixed() {
        assert_eq!(process_line("北京Beijing", false, true), "bjBeijing");
    }

    #[test]
    fn test_empty() {
        assert_eq!(process_line("", false, true), "");
    }

    #[test]
    fn test_pure_ascii() {
        assert_eq!(process_line("hello world", false, true), "hello world");
        assert_eq!(process_line("hello world", false, false), "");
    }
}
