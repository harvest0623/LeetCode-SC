type Answer struct {
    ans [3]int
}

func (this *Answer) put(x int) {
    if x > this.ans[0] {
        this.ans[2] = this.ans[1]
        this.ans[1] = this.ans[0]
        this.ans[0] = x
    } else if x != this.ans[0] && x > this.ans[1] {
        this.ans[2] = this.ans[1]
        this.ans[1] = x
    } else if x != this.ans[0] && x != this.ans[1] && x > this.ans[2] {
        this.ans[2] = x
    }
}

func (this *Answer) get() []int {
    var ret []int
    for _, num := range this.ans {
        if num != 0 {
            ret = append(ret, num)
        }
    }
    return ret
}

func getBiggestThree(grid [][]int) []int {
    m, n := len(grid), len(grid[0])
    sum1 := make([][]int, m + 1)
    sum2 := make([][]int, m + 1)
    for i := 0; i <= m; i++ {
        sum1[i] = make([]int, n + 2)
        sum2[i] = make([]int, n + 2)
    }
    
    for i := 1; i <= m; i++ {
        for j := 1; j <= n; j++ {
            sum1[i][j] = sum1[i-1][j-1] + grid[i-1][j-1]
            sum2[i][j] = sum2[i-1][j+1] + grid[i-1][j-1]
        }
    }
    
    ans := Answer{}
    for i := 0; i < m; i++ {
        for j := 0; j < n; j++ {
            // 单独的一个格子也是菱形
            ans.put(grid[i][j])
            for k := i + 2; k < m; k += 2 {
                ux, uy := i, j
                dx, dy := k, j
                lx, ly := (i + k) / 2, j - (k - i) / 2
                rx, ry := (i + k) / 2, j + (k - i) / 2
                if ly < 0 || ry >= n {
                    break
                }
                sum := (sum2[lx + 1][ly + 1] - sum2[ux][uy + 2]) +
                       (sum1[rx + 1][ry + 1] - sum1[ux][uy]) +
                       (sum1[dx + 1][dy + 1] - sum1[lx][ly]) +
                       (sum2[dx + 1][dy + 1] - sum2[rx][ry + 2]) -
                       (grid[ux][uy] + grid[dx][dy] + grid[lx][ly] + grid[rx][ry])

                ans.put(sum)
            }
        }
    }
    
    return ans.get()
}