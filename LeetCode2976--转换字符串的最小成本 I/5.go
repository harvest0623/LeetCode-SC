func minimumCost(source string, target string, original []byte, changed []byte, cost []int) int64 {
    g := make([][]int, 26)
    for i := range g {
        g[i] = make([]int, 26)
        for j := range g[i] {
            if i == j {
                g[i][j] = 0
            } else {
                g[i][j] = 0x3f3f3f3f
            }
        }
    }
    for i := 0; i < len(original); i++ {
        x := int(original[i] - 'a')
        y := int(changed[i] - 'a')
        if cost[i] < g[x][y] {
            g[x][y] = cost[i]
        }
    }
    for k := 0; k < 26; k++ {
        for i := 0; i < 26; i++ {
            if g[i][k] >= 0x3f3f3f3f {
                continue
            }
            for j := 0; j < 26; j++ {
                if g[i][k]+g[k][j] < g[i][j] {
                    g[i][j] = g[i][k] + g[k][j]
                }
            }
        }
    }
    ans := 0
    for i := 0; i < len(source); i++ {
        a := int(source[i] - 'a')
        b := int(target[i] - 'a')
        if g[a][b] >= 0x3f3f3f3f {
            return -1
        }
        ans += g[a][b]
    }
    return int64(ans)
}