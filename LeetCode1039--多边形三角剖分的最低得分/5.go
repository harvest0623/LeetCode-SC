func minScoreTriangulation(values []int) int {
    n := len(values)
    dp := make([][]int, n)
    for i := range dp {
        dp[i] = make([]int, n)
    }
    for length := 2; length < n; length++ {
        for i := 0; i+length < n; i++ {
            j := i + length
            dp[i][j] = 1 << 31 - 1 // MaxInt32
            for k := i + 1; k < j; k++ {
                score := dp[i][k] + dp[k][j] + values[i] * values[k] * values[j]
                if score < dp[i][j] {
                    dp[i][j] = score
                }
            }
        }
    }
    return dp[0][n-1]
}