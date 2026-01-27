func countTriples(n int) int {
    res := 0
    // 枚举 a 与 b
    for a := 1; a <= n; a++ {
        for b := 1; b <= n; b++ {
            // 判断是否符合要求
            c := int(math.Sqrt(float64(a * a + b * b + 1)))
            if c <= n && c * c == a * a + b * b {
                res++
            }
        }
    }
    return res
}