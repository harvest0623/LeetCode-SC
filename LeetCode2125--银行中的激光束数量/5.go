func numberOfBeams(bank []string) int {
    last, ans := 0, 0
    for _, line := range bank {
        cnt := strings.Count(line, "1")
        if cnt != 0 {
            ans += last * cnt
            last = cnt
        }
    }
    return ans
}