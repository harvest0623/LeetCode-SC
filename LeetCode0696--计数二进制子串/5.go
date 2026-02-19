func countBinarySubstrings(s string) int {
    pre, cnt, ans := 0, 1, 0
    for i := 0; i < len(s)-1; i++ {
        if s[i] == s[i+1] {
            cnt++
        } else {
            ans += min(pre, cnt)
            pre = cnt
            cnt = 1
        }
    }
    ans += min(pre, cnt)
    return ans
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}