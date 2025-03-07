pub fn odd_fibnacci_sum(n: u32) -> u32 {
    if n <= 1 {
        return 0; // 小于 1 无奇数
    }

    let mut sum = 0;
    let mut a = 0; // F(0)
    let mut b = 1; // F(1)

    // 生成斐波那契数，直到超过 n
    while a < n {
        // 如果是奇数，加入总和
        if a % 2 == 1 {
            sum += a;
        }

        // 计算下一项
        let next = a + b;
        a = b;
        b = next;
    }

    sum
}