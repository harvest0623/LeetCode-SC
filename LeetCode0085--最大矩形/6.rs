// 方法一: 使用柱状图的优化暴力方法
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        let mut left = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    left[i][j] = if j == 0 { 0 } else { left[i][j - 1] } + 1;
                }
            }
        }
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '0' {
                    continue;
                }
                let mut width = left[i][j];
                let mut area = width;
                for k in (0..i).rev() {
                    width = width.min(left[k][j]);
                    area = area.max((i - k + 1) * width);
                }
                ret = ret.max(area);
            }
        }
        ret as i32
    }
}

// 方法二：单调栈
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        let mut left = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    left[i][j] = if j == 0 { 0 } else { left[i][j - 1] } + 1;
                }
            }
        }

        let mut ret = 0;
        for j in 0..n {
            let mut up = vec![0; m];
            let mut down = vec![0; m];
            let mut stk: Vec<usize> = Vec::new();

            for i in 0..m {
                while let Some(&top) = stk.last() {
                    if left[top][j] >= left[i][j] {
                        stk.pop();
                    } else {
                        break;
                    }
                }
                up[i] = stk.last().map(|&x| x as i32).unwrap_or(-1);
                stk.push(i);
            }

            stk.clear();
            for i in (0..m).rev() {
                while let Some(&top) = stk.last() {
                    if left[top][j] >= left[i][j] {
                        stk.pop();
                    } else {
                        break;
                    }
                }
                down[i] = stk.last().copied().unwrap_or(m);
                stk.push(i);
            }

            for i in 0..m {
                let height = down[i] - up[i] as usize - 1;
                ret = ret.max(height * left[i][j]);
            }
        }

        ret as i32
    }
}