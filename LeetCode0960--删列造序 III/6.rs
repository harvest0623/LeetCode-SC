impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs[0].len();
        let mut dp = vec![1; n];
        for i in (0..n-1).rev() {
            for j in i+1..n {
                let mut valid = true;
                for row in &strs {
                    let char_i = row.chars().nth(i).unwrap();
                    let char_j = row.chars().nth(j).unwrap();
                    if char_i > char_j {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    dp[i] = dp[i].max(1 + dp[j]);
                }
            }
        }
        let max_dp = dp.iter().max().unwrap();
        (n - max_dp) as i32
    }
}