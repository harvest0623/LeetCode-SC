impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_prefix = vec![vec![0; n + 1]; m + 1];
        let mut col_prefix = vec![vec![0; n + 1]; m + 1];
        
        for i in 0..m {
            for j in 0..n {
                row_prefix[i + 1][j + 1] = row_prefix[i + 1][j] + grid[i][j];
                col_prefix[i + 1][j + 1] = col_prefix[i][j + 1] + grid[i][j];
            }
        }
        
        let get_row_sum = |r: usize, c1: usize, c2: usize| -> i32 {
            row_prefix[r + 1][c2 + 1] - row_prefix[r + 1][c1]
        };
        
        let get_col_sum = |c: usize, r1: usize, r2: usize| -> i32 {
            col_prefix[r2 + 1][c + 1] - col_prefix[r1][c + 1]
        };
        
        for k in (2..=m.min(n)).rev() {
            for r in 0..=m - k {
                for c in 0..=n - k {
                    let target = get_row_sum(r, c, c + k - 1);
                    let mut ok = true;
                    
                    for i in r..r + k {
                        if get_row_sum(i, c, c + k - 1) != target {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        continue;
                    }
                    
                    for j in c..c + k {
                        if get_col_sum(j, r, r + k - 1) != target {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        continue;
                    }
                    
                    let mut diag1 = 0;
                    let mut diag2 = 0;
                    for d in 0..k {
                        diag1 += grid[r + d][c + d];
                        diag2 += grid[r + d][c + k - 1 - d];
                    }
                    if diag1 != target || diag2 != target {
                        continue;
                    }
                    
                    return k as i32;
                }
            }
        }
        1
    }
}