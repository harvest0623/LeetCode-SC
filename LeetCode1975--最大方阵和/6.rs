impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let n = matrix.len();
        let mut cnt = 0;   // 负数元素的数量
        let mut total: i64 = 0;   // 所有元素的绝对值之和
        let mut mn = i32::MAX;   // 方阵元素的最小绝对值
        for i in 0..n {
            for j in 0..n {
                let abs_val = matrix[i][j].abs();
                mn = mn.min(abs_val);
                if matrix[i][j] < 0 {
                    cnt += 1;
                }
                total += abs_val as i64;
            }
        }
        // 按照负数元素的数量的奇偶性讨论
        if cnt % 2 == 0 {
            total
        } else {
            total - 2 * mn as i64
        }
    }
}