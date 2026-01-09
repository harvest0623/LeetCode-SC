func findMaxForm(strs []string, m, n int) int {
    length := len(strs)
    dp := make([][][]int, length+1)
    for i := range dp {
        dp[i] = make([][]int, m+1)
        for j := range dp[i] {
            dp[i][j] = make([]int, n+1)
        }
    }
    for i, s := range strs {
        zeros := strings.Count(s, "0")
        ones := len(s) - zeros
        for j := 0; j <= m; j++ {
            for k := 0; k <= n; k++ {
                dp[i+1][j][k] = dp[i][j][k]
                if j >= zeros && k >= ones {
                    dp[i+1][j][k] = max(dp[i+1][j][k], dp[i][j-zeros][k-ones]+1)
                }
            }
        }
    }
    return dp[length][m][n]
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}