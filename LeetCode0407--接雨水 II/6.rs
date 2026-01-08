use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let mut h = BinaryHeap::new();
        for (i, row) in height_map.iter_mut().enumerate() {
            for (j, height) in row.iter_mut().enumerate() {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    h.push((-*height, i, j)); // 取相反数变成最小堆
                    *height = -1; // 标记 (i,j) 访问过
                }
            }
        }

        let mut ans = 0;
        while let Some((min_height, i, j)) = h.pop() { 
            let min_height = -min_height; 
            for (x, y) in [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
                if x < m && y < n && height_map[x][y] >= 0 { 
                    ans += 0.max(min_height - height_map[x][y]);
                    h.push((-min_height.max(height_map[x][y]), x, y));
                    height_map[x][y] = -1; 
                }
            }
        }
        ans
    }
}