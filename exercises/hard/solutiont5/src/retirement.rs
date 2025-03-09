use std::cmp::min;

// 退休政策参数
#[derive(Debug)]
struct RetirementPolicy {
    original_retirement_age: i32, // 原退休年龄（年）
    delay_start_year: i32,        // 延迟政策起始年
    max_delay_months: i32,        // 最大延迟月数
}

// 定义人员类型对应的政策
const MALE_WORKER: RetirementPolicy = RetirementPolicy {
    original_retirement_age: 60,
    delay_start_year: 1965,
    max_delay_months: 36,
};
const FEMALE_55: RetirementPolicy = RetirementPolicy {
    original_retirement_age: 55,
    delay_start_year: 1970,
    max_delay_months: 36,
};
const FEMALE_50: RetirementPolicy = RetirementPolicy {
    original_retirement_age: 50,
    delay_start_year: 1975,
    max_delay_months: 60,
};

// 计算退休信息
fn calculate_retirement(
    birth_year: i32,
    birth_month: i32,
    policy: &RetirementPolicy,
) -> (i32, i32, f64, i32) {
    // 如果出生年早于延迟起始年，无延迟
    if birth_year < policy.delay_start_year {
        let retire_year = birth_year + policy.original_retirement_age;
        return (
            retire_year,
            birth_month,
            policy.original_retirement_age as f64,
            0,
        );
    }

    // 计算从延迟起始年到出生年的月数
    let months_from_start = (birth_year - policy.delay_start_year) * 12 + birth_month - 1;
    // 每 4 个月延迟 1 个月，取最大值限制
    let delay_months = min(months_from_start / 4 + 1, policy.max_delay_months);
    // 计算退休总月数（从延迟起始年算起）
    let retire_months_from_start =
        months_from_start + policy.original_retirement_age * 12 + delay_months;
    // 转换为退休年月
    let retire_year = policy.delay_start_year + retire_months_from_start / 12;
    let retire_month = retire_months_from_start % 12 + 1;
    // 计算退休年龄（年）
    let retire_age_months = retire_months_from_start - months_from_start;
    let retire_age = retire_age_months as f64 / 12.0;

    (retire_year, retire_month, retire_age, delay_months)
}

// 格式化退休信息
fn format_retirement(retire_year: i32, retire_month: i32, retire_age: f64, delay_months: i32) -> String {
    let age_str = if retire_age.fract() == 0.0 {
        format!("{:.0}", retire_age)
    } else {
        format!("{:.2}", retire_age)
    };
    format!("{:04}-{:02},{},{}", retire_year, retire_month, age_str, delay_months)
}

/// 计算退休时间
/// 输入: time 如 "1971-04", tp 如 "男职工"
/// 输出: 如 "2026-08,55.33,4"
pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生年月
    let parts: Vec<&str> = time.split('-').collect();
    if parts.len() != 2 {
        return "无效时间格式".to_string();
    }
    let birth_year = match parts[0].parse::<i32>() {
        Ok(y) => y,
        Err(_) => return "无效年份".to_string(),
    };
    let birth_month = match parts[1].parse::<i32>() {
        Ok(m) => m,
        Err(_) => return "无效月份".to_string(),
    };

    // 匹配人员类型
    let policy = match tp {
        "男职工" => &MALE_WORKER,
        "原法定退休年龄55周岁女职工" => &FEMALE_55,
        "原法定退休年龄50周岁女职工" => &FEMALE_50,
        _ => return "未知人员类型".to_string(),
    };

    // 计算并格式化
    let (retire_year, retire_month, retire_age, delay_months) =
        calculate_retirement(birth_year, birth_month, policy);
    format_retirement(retire_year, retire_month, retire_age, delay_months)
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::retire_time;

    #[test]
    fn test_retire_time() {
        let cases = [
            ("1971-04", "原法定退休年龄55周岁女职工", "2026-08,55.33,4"),
            ("1995-12", "原法定退休年龄50周岁女职工", "2050-12,55,60"),
            ("1995-12", "男职工", "2058-12,63,36"),
            ("2000-12", "原法定退休年龄55周岁女职工", "2058-12,58,36"),
        ];
        for (time, tp, expected) in cases {
            let result = retire_time(time, tp);
            assert_eq!(result, expected, "Input: {}, {}, Expected: {}, Got: {}", time, tp, expected, result);
        }
    }
}