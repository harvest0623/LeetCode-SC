func numSteps(s string) int {
    ans := len(s) - 1
    i := strings.LastIndex(s, "1")
    if i > 0 {
        count := 0
        for j := 1; j < i; j++ {
            if s[j] == '0' {
                count++
            }
        }
        ans += count + 2
    }
    return ans
}