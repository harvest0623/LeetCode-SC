impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut pos = vec![-1; n];
        
        for i in 0..n {
            for j in (0..n).rev() {
                if grid[i][j] == 1 {
                    pos[i] = j as i32;
                    break;
                }
            }
        }
        
        let mut ans = 0;
        let mut pos = pos;
        for i in 0..n {
            let mut k = -1;
            for j in i..n {
                if pos[j] <= i as i32 {
                    ans += j - i;
                    k = j as i32;
                    break;
                }
            }
            
            if k != -1 {
                let k = k as usize;
                for j in (i + 1..= k).rev() {
                    pos.swap(j, j - 1);
                }
            } else {
                return -1;
            }
        }
        ans as i32
    }
}