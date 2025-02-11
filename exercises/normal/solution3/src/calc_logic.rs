pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0;
    }

    // 计算没有任何两个人生日相同的概率
    let mut prob = 1.0;
    for i in 0..n {
        prob *= (365 - i) as f64 / 365f64;
    }

    // 计算至少有两个人生日相同的概率
    1.0 - prob
}
