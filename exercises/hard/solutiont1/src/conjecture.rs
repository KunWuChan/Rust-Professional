pub fn goldbach_conjecture() -> String {
    // 埃氏筛生成素数
    const LIMIT: usize = 6000; // 超过 5993
    let mut is_prime = vec![true; LIMIT];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..LIMIT {
        if is_prime[i] {
            for j in (i * i..LIMIT).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    // 检查奇合数
    let mut counter_examples = Vec::new();
    for n in (9..LIMIT).step_by(2) { // 从最小奇合数 9 开始
        if is_prime[n] {
            continue; // 跳过素数
        }
        let mut has_solution = false;
        for k in 1.. {
            let square_term = 2 * k * k;
            if square_term >= n {
                break;
            }
            let p = n - square_term;
            if p > 0 && is_prime[p] {
                has_solution = true;
                break;
            }
        }
        if !has_solution {
            counter_examples.push(n);
            if counter_examples.len() == 2 {
                break;
            }
        }
    }

    // 拼接结果
    format!("{},{}", counter_examples[0], counter_examples[1])
}