func bestClosingTime(customers string) int {
    n := len(customers)
    suf := 0
    pre := 0
    minCost := 0
    res := 0
    
    for i := 0; i <= n; i++ {
        if minCost > suf + pre {
            minCost = suf + pre
            res = i
        }
        if i < n && customers[i] == 'N' {
            pre++
        } else if i < n {
            suf--
        }
    }
    return res
}