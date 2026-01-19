// 方法一：遍历 (O(m × n))
fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    for row in grid {
        for val in row {
            if val < 0 {
                cnt += 1;
            }
        }
    }
    cnt
}

// 方法二：有序性 (O(m + n))
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let (mut r, mut c) = (0, grid[0].len() as i32 - 1);
        let mut cnt = 0;
        while r < grid.len() as i32 && c >= 0 {
            if grid[r as usize][c as usize] < 0 {
                cnt += grid.len() as i32 - r;
                c -= 1;
            } else {
                r += 1;
            }
        }
        cnt
    }
}

// 使用迭代器的简洁版本
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = 0;
        for row in grid {
            cnt += row.iter().filter(|&&val| val < 0).count() as i32;
        }
        cnt
    }
}