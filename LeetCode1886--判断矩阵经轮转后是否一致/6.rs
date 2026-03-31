impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        let mut mat = mat;
        
        // 最多旋转 4 次
        for _ in 0..4 {
            // 旋转操作
            for i in 0..n/2 {
                for j in 0..(n+1)/2 {
                    let temp = mat[i][j];
                    mat[i][j] = mat[n-1-j][i];
                    mat[n-1-j][i] = mat[n-1-i][n-1-j];
                    mat[n-1-i][n-1-j] = mat[j][n-1-i];
                    mat[j][n-1-i] = temp;
                }
            }
            
            if mat == target {
                return true;
            }
        }
        false
    }
}