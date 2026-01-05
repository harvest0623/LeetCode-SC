func myPow(x float64, n int) float64 {
    if n < 0 {
        return myPow(1/x, -n)
    }
    if n == 0 {
        return 1
    }
    res := myPow(x, n/2)
    res *= res
    if n%2 != 0 {
        res *= x
    }
    return res
}