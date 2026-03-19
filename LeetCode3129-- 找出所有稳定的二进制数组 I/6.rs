const MOD: i32 = 1000000007;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let mut dp = vec![vec![vec![0; 2]; one as usize + 1]; zero as usize + 1];

        for i in 0..=zero.min(limit) as usize {
            dp[i][0][0] = 1;
        }
        for j in 0..=one.min(limit) as usize {
            dp[0][j][1] = 1;
        }

        for i in 1..=zero as usize {
            for j in 1..=one as usize {
                if i > limit as usize {
                    dp[i][j][0] = dp[i - 1][j][0] + dp[i - 1][j][1] - dp[i - limit as usize - 1][j][1];
                } else {
                    dp[i][j][0] = dp[i - 1][j][0] + dp[i - 1][j][1];
                }
                dp[i][j][0] = (dp[i][j][0] % MOD + MOD) % MOD;
                if j > limit as usize {
                    dp[i][j][1] = dp[i][j - 1][1] + dp[i][j - 1][0] - dp[i][j - limit as usize - 1][0];
                } else {
                    dp[i][j][1] = dp[i][j - 1][1] + dp[i][j - 1][0];
                }
                dp[i][j][1] = (dp[i][j][1] % MOD + MOD) % MOD;
            }
        }
        (dp[zero as usize][one as usize][0] + dp[zero as usize][one as usize][1]) % MOD
    }
}