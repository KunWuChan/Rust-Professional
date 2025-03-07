pub fn dp_rec_mc(amount: u32) -> u32 {
// 币种数组
    const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];

// 金额为 0 时无需纸币
    if amount == 0 {
        return 0;
    }

    // dp[i] 表示金额 i 所需的最少纸币数
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0; // 基例

    // 填充 dp 数组
    for i in 1..=amount {
        for &coin in CASHES.iter() {
            if i >= coin && dp[(i - coin) as usize] != u32::MAX {
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
            }
        }
    }

    // 返回结果，若无法找零则返回 u32::MAX（但测试用例保证有解）
    dp[amount as usize]
}