const DIRS: [(i8, i8); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(i: usize, j: usize, vis: &mut Vec<Vec<bool>>, heights: &Vec<Vec<i32>>) {
            if vis[i][j] { 
                return;
            }
            vis[i][j] = true; 
            for (dx, dy) in DIRS { 
                let x = i + dx as usize;
                let y = j + dy as usize;
                if x < heights.len() && y < heights[i].len() && heights[x][y] >= heights[i][j] { // 往高处走
                    dfs(x, y, vis, heights);
                }
            }
        }
        let m = heights.len();
        let n = heights[0].len();
        let mut pacific_vis = vec![vec![false; n]; m];
        for j in 0..n {
            dfs(0, j, &mut pacific_vis, &heights); 
        }
        for i in 1..m {
            dfs(i, 0, &mut pacific_vis, &heights);
        }
        let mut atlantic_vis = vec![vec![false; n]; m];
        for j in 0..n {
            dfs(m - 1, j, &mut atlantic_vis, &heights); 
        }
        for i in 0..m - 1 {
            dfs(i, n - 1, &mut atlantic_vis, &heights); 
        }
        let mut ans = vec![];
        for i in 0..m {
            for j in 0..n {
                if pacific_vis[i][j] && atlantic_vis[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }
}