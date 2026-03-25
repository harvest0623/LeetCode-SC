func getHappyString(n int, k int) string {
    chs := []byte{'a', 'b', 'c'}
    res := make([]byte, 0, n)
    if k > 3 * (1 << (n - 1)) {
        return string(res)
    }
    for i := 0; i < n; i++ {
        count := 1 << (n - i - 1)
        for _, c := range chs {
            if len(res) > 0 && res[len(res) - 1] == c {
                continue
            }
            if k <= count {
                res = append(res, c)
                break
            }
            k -= count
        }
    }
    return string(res)
}