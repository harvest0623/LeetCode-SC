func minimumDeletions(s string) int {
    ans := 0
    cntA, cnt := 0, 0
    for _, c := range s {
        if c == 'a' {
            cnt--
            cntA++
        } else {
            cnt++
        }
        if cnt < ans {
            ans = cnt
        }
    }
    return cntA + ans
}