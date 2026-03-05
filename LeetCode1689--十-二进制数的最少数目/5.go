func minPartitions(n string) int {
    res := 0
    for _, c := range n {
        res = max(res, int(c - '0'))
    }
    return res
}