pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (num, base) = parse_input(num_str);
    let decimal_value = from_base(num, base);
    if decimal_value == 0 {
        return "0".to_string();
    }
    let mut result = String::new();
    let mut n = decimal_value;
    while n > 0 {
        let remainder = n % to_base;
        let digit = match remainder {
            0..=9 => char::from_digit(remainder, 10).unwrap(),
            10..=16 => char::from_digit(remainder, 16).unwrap(),
            _ => unreachable!(),
        };
        result.push(digit);
        n /= to_base
    }

    result.chars().rev().collect()
}

fn from_base(num_str: &str, base: u32) -> u32 {
    let mut result = 0;
    for (i, digit) in num_str.chars().rev().enumerate() {
        let digit_value = match digit {
            '0'..='9' => digit.to_digit(10).unwrap(),
            'a'..='f' => digit.to_digit(16).unwrap(),
            'A'..='F' => digit.to_digit(16).unwrap(),
            _ => panic!("Invalid character in input string"),
        };
        result += digit_value * base.pow(i as u32);
    }

    result
}

// 解析输入字符串：提取数字和进制
fn parse_input(input: &str) -> (&str, u32) {
    let parts: Vec<&str> = input.split("(").collect();
    let numer = parts[0];
    let base = parts[1].trim_end_matches(")").parse().unwrap();
    (numer, base)
}
