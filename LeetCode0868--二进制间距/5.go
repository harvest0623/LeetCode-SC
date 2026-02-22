func binaryGap(n int) (ans int) {
    for i, last := 0, -1; n > 0; i++ {
        if n&1 == 1 {
            if last != -1 {
                ans = max(ans, i-last)
            }
            last = i
        }
        n >>= 1
    }
    return
}

func max(a, b int) int {
    if b > a {
        return b
    }
    return a
}