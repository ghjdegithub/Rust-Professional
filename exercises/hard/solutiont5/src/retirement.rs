pub fn retire_time(time: &str, tp: &str) -> String {
    let (retirement_date, retirement_age, delay_months) = calculate_retirement(time, tp);
    format!(
        "{},{},{:.0}",
        retirement_date,
        format_age(retirement_age),
        delay_months
    )
}

fn calculate_retirement(birth_date: &str, employee_type: &str) -> (String, f64, i32) {
    let (birth_year, birth_month) = parse_date(birth_date);
    let (base_retirement_age, delay_rate, max_delay) = match employee_type {
        "男职工" => (60, 1.0 / 4.0, 36),
        "原法定退休年龄55周岁女职工" => (55, 1.0 / 4.0, 36),
        "原法定退休年龄50周岁女职工" => (50, 1.0 / 2.0, 60),
        _ => panic!("未知的员工类型"),
    };

    let base_retirement_year = birth_year + base_retirement_age;
    let delay_months =
        calculate_delay_months(base_retirement_year, birth_month, delay_rate, max_delay);
    let (retirement_year, retirement_month) =
        add_months(base_retirement_year, birth_month, delay_months);
    let retirement_age = base_retirement_age as f64 + delay_months as f64 / 12.0;

    (
        format!("{:04}-{:02}", retirement_year, retirement_month),
        retirement_age,
        delay_months,
    )
}

fn parse_date(date: &str) -> (i32, i32) {
    let parts: Vec<&str> = date.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<i32>().unwrap();
    (year, month)
}

fn calculate_delay_months(
    base_retirement_year: i32,
    birth_month: i32,
    delay_rate: f64,
    max_delay: i32,
) -> i32 {
    let start_delay_year = 2025;
    let start_delay_month = 0;

    if base_retirement_year < start_delay_year
        || (base_retirement_year == start_delay_year && birth_month < start_delay_month)
    {
        return 0;
    }

    let months_since_start =
        (base_retirement_year - start_delay_year) * 12 + (birth_month - start_delay_month);
    let delay_months = (months_since_start as f64 * delay_rate).ceil() as i32;

    if delay_months > max_delay {
        max_delay
    } else {
        delay_months
    }
}

fn add_months(year: i32, month: i32, months_to_add: i32) -> (i32, i32) {
    let total_months = year * 12 + month + months_to_add;
    let new_year = total_months / 12;
    let new_month = total_months % 12;
    if new_month == 0 {
        (new_year - 1, 12)
    } else {
        (new_year, new_month)
    }
}

fn format_age(age: f64) -> String {
    if age.fract() == 0.0 {
        format!("{:.0}", age)
    } else {
        format!("{:.2}", age)
    }
}
