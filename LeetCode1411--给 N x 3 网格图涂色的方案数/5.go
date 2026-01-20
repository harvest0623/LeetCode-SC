func numOfWays(n int) int {
    mod := 1e9 + 7
    preA, preB := 6, 6
    for i := 2; i <= n; i++ {
        new_preA := (2*preA + 2*preB) % mod
        new_preB := (2*preA + 3*preB) % mod
        preA, preB = new_preA, new_preB
    }
    return (preA + preB) % mod
}