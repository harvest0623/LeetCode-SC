type pair struct{ x, y int }
var dirs = []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func swimInWater(grid [][]int) (ans int) {
    n := len(grid)
    return sort.Search(n*n-1, func(threshold int) bool {
        if threshold < grid[0][0] {
            return false
        }
        vis := make([][]bool, n)
        for i := range vis {
            vis[i] = make([]bool, n)
        }
        vis[0][0] = true
        queue := []pair{{}}
        for len(queue) > 0 {
            p := queue[0]
            queue = queue[1:]
            for _, d := range dirs {
                if x, y := p.x+d.x, p.y+d.y; 0 <= x && x < n && 0 <= y && y < n && !vis[x][y] && grid[x][y] <= threshold {
                    vis[x][y] = true
                    queue = append(queue, pair{x, y})
                }
            }
        }
        return vis[n-1][n-1]
    })
}