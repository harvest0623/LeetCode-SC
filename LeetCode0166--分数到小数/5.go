func fractionToDecimal(numerator, denominator int) string {
    sign := ""
    if numerator*denominator < 0 {
        sign = "-"
    }
    numerator = abs(numerator) 
    denominator = abs(denominator)
    q, r := numerator/denominator, numerator%denominator
    if r == 0 { 
        return sign + strconv.Itoa(q)
    }
    ans := []byte(sign + strconv.Itoa(q) + ".")
    rToPos := map[int]int{r: len(ans)}
    for r != 0 {
        r *= 10
        q = r / denominator
        r %= denominator
        ans = append(ans, '0'+byte(q))
        if pos, ok := rToPos[r]; ok {
            return string(ans[:pos]) + "(" + string(ans[pos:]) + ")"
        }
        rToPos[r] = len(ans) 
    }
    return string(ans) 
}
func abs(x int) int { if x < 0 { return -x }; return x }