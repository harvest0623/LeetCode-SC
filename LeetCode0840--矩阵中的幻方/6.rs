impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        
        if rows < 3 || cols < 3 {
            return 0;
        }
        
        let mut count = 0;
        for r in 0..=rows - 3 {
            for c in 0..=cols - 3 {
                if Self::is_magic_square(&grid, r, c) {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    fn is_magic_square(grid: &Vec<Vec<i32>>, r: usize, c: usize) -> bool {
        // 中心必须是 5
        if grid[r + 1][c + 1] != 5 {
            return false;
        }
        
        // 检查是否为 1~9 不重复
        let mut seen = [false; 10];
        for i in r..r + 3 {
            for j in c..c + 3 {
                let num = grid[i][j] as usize;
                if num < 1 || num > 9 || seen[num] {
                    return false;
                }
                seen[num] = true;
            }
        }
        
        // 检查行和
        if grid[r][c] + grid[r][c + 1] + grid[r][c + 2] != 15 { return false; }
        if grid[r + 1][c] + grid[r + 1][c + 1] + grid[r + 1][c + 2] != 15 { return false; }
        if grid[r + 2][c] + grid[r + 2][c + 1] + grid[r + 2][c + 2] != 15 { return false; }
        
        // 检查列和
        if grid[r][c] + grid[r + 1][c] + grid[r + 2][c] != 15 { return false; }
        if grid[r][c + 1] + grid[r + 1][c + 1] + grid[r + 2][c + 1] != 15 { return false; }
        if grid[r][c + 2] + grid[r + 1][c + 2] + grid[r + 2][c + 2] != 15 { return false; }
        
        // 检查对角线
        if grid[r][c] + grid[r + 1][c + 1] + grid[r + 2][c + 2] != 15 { return false; }
        if grid[r][c + 2] + grid[r + 1][c + 1] + grid[r + 2][c] != 15 { return false; }
        
        true
    }
}