impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn dfs(i: usize, memo: &mut Vec<i32>) -> i32 {
            if i <= 1 {
                return 1;
            }
            if memo[i] != 0 { 
                return memo[i];
            }
            let res = dfs(i - 1, memo) + dfs(i - 2, memo);
            memo[i] = res; 
            res
        }
        let n = n as usize;
        let mut memo = vec![0; n + 1];
        dfs(n, &mut memo)
    }
}

// 优化
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n + 1];
        f[0] = 1;
        f[1] = 1;
        for i in 2..=n {
            f[i] = f[i - 1] + f[i - 2];
        }
        f[n]
    }
}