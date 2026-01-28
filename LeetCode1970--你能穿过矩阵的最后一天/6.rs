use std::collections::VecDeque;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (row, col) = (row as usize, col as usize);
        
        let mut left = 0;
        let mut right = row * col;
        let mut ans = 0;
        
        while left <= right {
            let mid = (left + right) / 2;
            let mut grid = vec![vec![1; col]; row];
            for i in 0..mid {
                let r = (cells[i][0] - 1) as usize;
                let c = (cells[i][1] - 1) as usize;
                grid[r][c] = 0;
            }
            
            let mut queue = VecDeque::new();
            for i in 0..col {
                if grid[0][i] == 1 {
                    queue.push_back((0, i));
                    grid[0][i] = 0;
                }
            }
            
            let mut found = false;
            while let Some((x, y)) = queue.pop_front() {
                for (dx, dy) in &dirs {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    
                    if nx >= 0 && nx < row as i32 && ny >= 0 && ny < col as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if grid[nx][ny] == 1 {
                            if nx == row - 1 {
                                found = true;
                                break;
                            }
                            queue.push_back((nx, ny));
                            grid[nx][ny] = 0;
                        }
                    }
                }
                if found {
                    break;
                }
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