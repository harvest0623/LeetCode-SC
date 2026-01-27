func largestMagicSquare(grid [][]int) int {
    m, n := len(grid), len(grid[0])
    rowPrefix := make([][]int, m+1)
    colPrefix := make([][]int, m+1)
    for i := 0; i <= m; i++ {
        rowPrefix[i] = make([]int, n+1)
        colPrefix[i] = make([]int, n+1)
    }
    
    for i := 0; i < m; i++ {
        for j := 0; j < n; j++ {
            rowPrefix[i+1][j+1] = rowPrefix[i+1][j] + grid[i][j]
            colPrefix[i+1][j+1] = colPrefix[i][j+1] + grid[i][j]
        }
    }
    
    getRowSum := func(r, c1, c2 int) int {
        return rowPrefix[r+1][c2+1] - rowPrefix[r+1][c1]
    }
    
    getColSum := func(c, r1, r2 int) int {
        return colPrefix[r2+1][c+1] - colPrefix[r1][c+1]
    }
    
    for k := min(m, n); k > 1; k-- {
        for r := 0; r+k <= m; r++ {
            for c := 0; c+k <= n; c++ {
                target := getRowSum(r, c, c+k-1)
                ok := true
                
                for i := r; i < r+k; i++ {
                    if getRowSum(i, c, c+k-1) != target {
                        ok = false
                        break
                    }
                }
                if !ok {
                    continue
                }
                
                for j := c; j < c+k; j++ {
                    if getColSum(j, r, r+k-1) != target {
                        ok = false
                        break
                    }
                }
                if !ok {
                    continue
                }
                
                diag1, diag2 := 0, 0
                for d := 0; d < k; d++ {
                    diag1 += grid[r+d][c+d]
                    diag2 += grid[r+d][c+k-1-d]
                }
                if diag1 != target || diag2 != target {
                    continue
                }
                
                return k
            }
        }
    }
    return 1
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}