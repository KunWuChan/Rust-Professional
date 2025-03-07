/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // 空字符串返回 0
    if s.is_empty() {
        return 0;
    }

    // 将字符串转为字符数组
    let chars: Vec<char> = s.chars().collect();
    let mut char_index: HashMap<char, usize> = HashMap::new(); // 记录字符最后出现的位置
    let mut max_len = 0; // 最长无重复子串长度
    let mut left = 0; // 窗口左边界

    // 遍历字符串，right 是右边界
    for right in 0..chars.len() {
        // 如果当前字符已出现，且位置在窗口内
        if let Some(&last_pos) = char_index.get(&chars[right]) {
            if last_pos >= left {
                left = last_pos + 1; // 移动左边界到重复字符后
            }
        }
        // 更新字符的最新位置
        char_index.insert(chars[right], right);
        // 更新最大长度
        max_len = max_len.max(right - left + 1);
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
