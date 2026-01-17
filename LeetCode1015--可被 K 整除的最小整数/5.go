func smallestRepunitDivByK(k int) int {
    if k % 2 == 0 || k % 5 == 0 {
        return -1
    }
    cur, res := 0, 1
    for {
        cur = (10 * cur + 1) % k
        if cur == 0 {
            return res
        }
        res++
    }
}