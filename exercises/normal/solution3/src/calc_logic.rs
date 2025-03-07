pub fn new_birthday_probability(n: u32) -> f64 {
    // 边界检查
    if n < 2 {
        return 0.0;
    }

    let days = 365.0; // 一年天数，用 f64 避免整数除法截断
    let mut prob_different = 1.0; // 所有人生日不同的概率

    // 计算 P(不同)
    for i in 0..n {
        let remaining = days - i as f64;
        prob_different *= remaining / days;

        // 若人数超过天数，概率为 1
        if remaining <= 0.0 {
            return 1.0;
        }
    }

    // P(至少两人相同) = 1 - P(不同)
    let prob_same = 1.0 - prob_different;

    // 保留四位小数
    (prob_same * 10000.0).round() / 10000.0
}
