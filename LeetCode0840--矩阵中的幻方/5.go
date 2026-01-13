func numMagicSquaresInside(grid [][]int) int {
    rows := len(grid)
    if rows == 0 {
        return 0
    }
    cols := len(grid[0])
    
    if rows < 3 || cols < 3 {
        return 0
    }
    
    count := 0
    for r := 0; r <= rows-3; r++ {
        for c := 0; c <= cols-3; c++ {
            if isMagicSquare(grid, r, c) {
                count++
            }
        }
    }
    
    return count
}

func isMagicSquare(grid [][]int, r int, c int) bool {
    // 中心必须是 5
    if grid[r+1][c+1] != 5 {
        return false
    }
    
    // 检查是否为 1~9 不重复
    seen := make([]bool, 10)
    for i := r; i < r+3; i++ {
        for j := c; j < c+3; j++ {
            num := grid[i][j]
            if num < 1 || num > 9 || seen[num] {
                return false
            }
            seen[num] = true
        }
    }
    
    // 检查行和
    if grid[r][c]+grid[r][c+1]+grid[r][c+2] != 15 {
        return false
    }
    if grid[r+1][c]+grid[r+1][c+1]+grid[r+1][c+2] != 15 {
        return false
    }
    if grid[r+2][c]+grid[r+2][c+1]+grid[r+2][c+2] != 15 {
        return false
    }
    
    // 检查列和
    if grid[r][c]+grid[r+1][c]+grid[r+2][c] != 15 {
        return false
    }
    if grid[r][c+1]+grid[r+1][c+1]+grid[r+2][c+1] != 15 {
        return false
    }
    if grid[r][c+2]+grid[r+1][c+2]+grid[r+2][c+2] != 15 {
        return false
    }
    
    // 检查对角线
    if grid[r][c]+grid[r+1][c+1]+grid[r+2][c+2] != 15 {
        return false
    }
    if grid[r][c+2]+grid[r+1][c+1]+grid[r+2][c] != 15 {
        return false
    }
    
    return true
}