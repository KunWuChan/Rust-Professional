use std::fmt::{self, Display, Formatter};

// 将任意进制字符串转换为十进制
fn to_decimal(num_str: &str, base: i32) -> Result<i32, String> {
    let mut result = 0;
    let is_negative = num_str.starts_with('-');
    let num_str = if is_negative { &num_str[1..] } else { num_str };

    for c in num_str.chars() {
        let digit = match c {
            '0'..='9' => (c as u8 - b'0') as i32,
            'A'..='F' => (c as u8 - b'A' + 10) as i32,
            'a'..='f' => (c as u8 - b'a' + 10) as i32,
            _ => return Err(format!("Invalid digit '{}' for base {}", c, base)),
        };
        if digit >= base {
            return Err(format!("Digit {} exceeds base {}", digit, base));
        }
        result = result * base + digit;
    }
    Ok(if is_negative { -result } else { result })
}

// 将十进制数转换为目标进制字符串（小写）
fn from_decimal(num: i32, target_base: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let is_negative = num < 0;
    let mut n = num.abs();
    let mut result = String::new();
    let digits = "0123456789abcdef"; // 改为小写

    while n > 0 {
        let digit = (n % target_base) as usize;
        result.insert(0, digits.chars().nth(digit).unwrap());
        n /= target_base;
    }
    if is_negative {
        result.insert(0, '-');
    }
    result
}

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let parts: Vec<&str> = num_str.split('(').collect();
    if parts.len() != 2 || !parts[1].ends_with(')') {
        return "0".to_string(); // 简化错误处理
    }

    let num = parts[0];
    let source_base = match parts[1][..parts[1].len() - 1].parse::<i32>() {
        Ok(b) => b,
        Err(_) => return "0".to_string(),
    };

    if source_base < 2 || source_base > 16 || to_base < 2 || to_base > 16 {
        return "0".to_string();
    }

    let decimal = match to_decimal(num, source_base) {
        Ok(d) => d,
        Err(_) => return "0".to_string(),
    };

    from_decimal(decimal, to_base as i32)
}