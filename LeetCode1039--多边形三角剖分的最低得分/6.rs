impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![0; n]; n];
        for len in 2..n {
            for i in 0..n - len {
                let j = i + len;
                dp[i][j] = i32::MAX;
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].min(
                        dp[i][k] + dp[k][j] + values[i] * values[k] * values[j]
                    );
                }
            }
        }
        dp[0][n - 1]
    }
}