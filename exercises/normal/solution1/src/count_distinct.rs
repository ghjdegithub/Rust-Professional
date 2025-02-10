use std::collections::{HashMap, HashSet};

pub fn new_count_distinct(input_str: &str) -> usize {
    // 将字符串按逗号分割成元素，并去除两端的空格
    let elements: Vec<&str> = input_str.split(',').map(|s| s.trim()).collect();

    // 使用 HashSet 存储不重复的元素
    let unique_elements: HashSet<&str> = elements.into_iter().collect();

    // 返回不重复元素的个数
    unique_elements.len()
}
