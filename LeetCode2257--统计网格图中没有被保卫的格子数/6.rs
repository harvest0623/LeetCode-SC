// 左右上下
const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut guarded = vec![vec![0i8; n]; m];

        // 标记警卫格子、墙格子
        for g in &guards {
            guarded[g[0] as usize][g[1] as usize] = -1;
        }
        for w in walls {
            guarded[w[0] as usize][w[1] as usize] = -1;
        }

        // 遍历警卫
        for g in guards {
            // 遍历视线方向（左右上下）
            for (dx, dy) in DIRS {
                // 视线所及之处，被保卫
                let mut x = (g[0] + dx) as usize;
                let mut y = (g[1] + dy) as usize;
                while x < m && y < n && guarded[x][y] != -1 {
                    guarded[x][y] = 1; // 被保卫
                    x += dx as usize;
                    y += dy as usize;
                }
            }
        }

        // 统计没被保卫的格子数
        guarded.into_iter().flatten().filter(|&x| x == 0).count() as _
    }
}