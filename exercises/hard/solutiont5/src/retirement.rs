pub fn retire_time(birth: &str, category: &str) -> String {
    // 手动解析 "YYYY-MM"
    let birth_year = birth[0..4].parse::<i32>().unwrap();
    let birth_month = birth[5..7].parse::<i32>().unwrap();

    // 类型参数
    let (original_age, months_per_increment, max_delay_months) = match category {
        "男职工" => (60, 4, 36),
        "原法定退休年龄55周岁女职工" => (55, 4, 36),
        "原法定退休年龄50周岁女职工" => (50, 2, 60),
        _ => panic!("未知人员类型"),
    };

    // 原退休时间
    let original_months = birth_year * 12 + birth_month + original_age * 12;
    static BASE_MONTHS: i32 = 2025 * 12 + 1; // 2025-01
    let months_from_base = original_months - BASE_MONTHS;
    // 延迟月数
    let delay_months = if months_from_base <= 0 {
        0
    } else {
        ((months_from_base + months_per_increment - 1) / months_per_increment).min(max_delay_months)
    };

    // 实际退休时间
    let total_months = months_from_base + delay_months;
    let final_retire_year = 2025 + total_months / 12;
    let final_retire_month = 1 + total_months % 12;
    let (adjusted_year, adjusted_month) = if final_retire_month > 12 {
        (final_retire_year + 1, final_retire_month - 12)
    } else {
        (final_retire_year, final_retire_month)
    };

    // 退休年龄
    let retire_age = original_age as f64 + (delay_months as f64 / 12.0);

    // 单次格式化
    if delay_months % 12 == 0 {
        format!("{:04}-{:02},{:.0},{}", adjusted_year, adjusted_month, retire_age, delay_months)
    } else {
        format!("{:04}-{:02},{:.2},{}", adjusted_year, adjusted_month, retire_age, delay_months)
    }
}