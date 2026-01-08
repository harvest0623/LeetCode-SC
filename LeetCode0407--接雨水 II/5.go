// 方法一：最小堆
func trapRainWater(heightMap [][]int) (ans int) {
    m, n := len(heightMap), len(heightMap[0])
    if m <= 2 || n <= 2 {
        return
    }

    vis := make([][]bool, m)
    for i := range vis {
        vis[i] = make([]bool, n)
    }
    h := &hp{}
    for i, row := range heightMap {
        for j, v := range row {
            if i == 0 || i == m-1 || j == 0 || j == n-1 {
                heap.Push(h, cell{v, i, j})
                vis[i][j] = true
            }
        }
    }

    dirs := []int{-1, 0, 1, 0, -1}
    for h.Len() > 0 {
        cur := heap.Pop(h).(cell)
        for k := 0; k < 4; k++ {
            nx, ny := cur.x+dirs[k], cur.y+dirs[k+1]
            if 0 <= nx && nx < m && 0 <= ny && ny < n && !vis[nx][ny] {
                if heightMap[nx][ny] < cur.h {
                    ans += cur.h - heightMap[nx][ny]
                }
                vis[nx][ny] = true
                heap.Push(h, cell{max(heightMap[nx][ny], cur.h), nx, ny})
            }
        }
    }
    return
}
type cell struct{ h, x, y int }
type hp []cell

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { return h[i].h < h[j].h }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(cell)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

func max(a, b int) int {
    if b > a {
        return b
    }
    return a
}

// 方法二：广度优先搜索
func trapRainWater(heightMap [][]int) (ans int) {
    m, n := len(heightMap), len(heightMap[0])
    maxHeight := 0
    for _, row := range heightMap {
        for _, h := range row {
            maxHeight = max(maxHeight, h)
        }
    }

    water := make([][]int, m)
    for i := range water {
        water[i] = make([]int, n)
        for j := range water[i] {
            water[i][j] = maxHeight
        }
    }
    type pair struct{ x, y int }
    q := []pair{}
    for i, row := range heightMap {
        for j, h := range row {
            if (i == 0 || i == m-1 || j == 0 || j == n-1) && h < water[i][j] {
                water[i][j] = h
                q = append(q, pair{i, j})
            }
        }
    }

    dirs := []int{-1, 0, 1, 0, -1}
    for len(q) > 0 {
        p := q[0]
        q = q[1:]
        x, y := p.x, p.y
        for i := 0; i < 4; i++ {
            nx, ny := x+dirs[i], y+dirs[i+1]
            if 0 <= nx && nx < m && 0 <= ny && ny < n && water[nx][ny] > water[x][y] && water[nx][ny] > heightMap[nx][ny] {
                water[nx][ny] = max(water[x][y], heightMap[nx][ny])
                q = append(q, pair{nx, ny})
            }
        }
    }

    for i, row := range heightMap {
        for j, h := range row {
            ans += water[i][j] - h
        }
    }
    return
}

func max(a, b int) int {
    if b > a {
        return b
    }
    return a
}