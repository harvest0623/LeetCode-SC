// 方法一：动态规划
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut f = vec![vec![0; n]; n];
        f[0][0] = triangle[0][0];
        for i in 1..n {
            f[i][0] = f[i - 1][0] + triangle[i][0];
            for j in 1..i {
                f[i][j] = std::cmp::min(f[i - 1][j - 1], f[i - 1][j]) + triangle[i][j];
            }
            f[i][i] = f[i - 1][i - 1] + triangle[i][i];
        }
        *f[n - 1].iter().min().unwrap()
    }
}

// 方法二：动态规划 + 空间优化
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut f = vec![0; n];
        f[0] = triangle[0][0];
        
        for i in 1..n {
            f[i] = f[i - 1] + triangle[i][i];
            for j in (1..i).rev() {
                f[j] = std::cmp::min(f[j - 1], f[j]) + triangle[i][j];
            }
            f[0] += triangle[i][0];
        }
        
        *f.iter().min().unwrap()
    }
}