func minimumDeleteSum(s1 string, s2 string) int {
    m, n := len(s1), len(s2)
    dp := make([][]int, m+1)
    for i := range dp {
        dp[i] = make([]int, n+1)
        if i > 0 {
            dp[i][0] = dp[i-1][0] + int(s1[i-1])
        }
    }
    for j := range dp[0] {
        if j > 0 {
            dp[0][j] = dp[0][j-1] + int(s2[j-1])
        }
    }
    for i, c1 := range s1 {
        for j, c2 := range s2 {
            if c1 == c2 {
                dp[i+1][j+1] = dp[i][j]
            } else {
                dp[i+1][j+1] = min(dp[i][j+1] + int(c1), dp[i+1][j] + int(c2))
            }
        }
    }
    return dp[m][n]
}