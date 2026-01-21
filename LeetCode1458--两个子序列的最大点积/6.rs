impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut f = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let xij = nums1[i] * nums2[j];
                f[i][j] = xij;
                if i > 0 {
                    f[i][j] = f[i][j].max(f[i - 1][j]);
                }
                if j > 0 {
                    f[i][j] = f[i][j].max(f[i][j - 1]);
                }
                if i > 0 && j > 0 {
                    f[i][j] = f[i][j].max(f[i - 1][j - 1] + xij);
                }
            }
        }
        f[m - 1][n - 1]
    }
}