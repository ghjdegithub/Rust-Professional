/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters.
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.

    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/
use std::cmp::max;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    let mut char_set = HashSet::new(); // 用于存储当前窗口中的字符
    let mut start = 0; // 窗口的起始位置
    let mut max_len = 0; // 最长不重复子串的长度

    for end in 0..s.len() {
        let c = s[end..=end].chars().next().unwrap(); // 获取当前字符

        // 如果字符已存在，移动 start 指针
        while char_set.contains(&c) {
            let start_char = s[start..=start].chars().next().unwrap();
            char_set.remove(&start_char);
            start += 1;
        }

        // 将当前字符加入到字符集
        char_set.insert(c);
        // 更新最长子串的长度
        max_len = max(max_len, end - start + 1)
    }

    max_len as i32 // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1); // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0); // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5); // "abcde"
    }
}
