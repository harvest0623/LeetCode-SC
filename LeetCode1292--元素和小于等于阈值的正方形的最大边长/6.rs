impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut prefix = vec![vec![0; n + 1]; m + 1];
        
        // 计算二维前缀和
        for i in 1..=m {
            for j in 1..=n {
                prefix[i][j] = prefix[i-1][j] + prefix[i][j-1] - prefix[i-1][j-1] + mat[i-1][j-1];
            }
        }
        
        // 二分查找
        let mut left = 0;
        let mut right = m.min(n);
        let mut ans = 0;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            let mut found = false;
            
            // 检查所有边长为mid的正方形
            for i in 1..=m {
                if i + mid > m + 1 { break; }
                for j in 1..=n {
                    if j + mid > n + 1 { break; }
                    let sum = prefix[i+mid-1][j+mid-1] - prefix[i-1][j+mid-1]
                            - prefix[i+mid-1][j-1] + prefix[i-1][j-1];
                    if sum <= threshold {
                        found = true;
                        break;
                    }
                }
                if found { break; }
            }           
            if found {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }       
        ans as i32
    }
}