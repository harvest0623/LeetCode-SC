func convert(s string, numRows int) string {
    n, r := len(s), numRows
    if r == 1 || r >= n {
        return s
    }
    t := r*2 - 2
    c := (n + t - 1) / t * (r - 1)
    mat := make([][]byte, r)
    for i := range mat {
        mat[i] = make([]byte, c)
    }
    x, y := 0, 0
    for i, ch := range s {
        mat[x][y] = byte(ch)
        if i%t < r-1 {
            x++ // 向下移动
        } else {
            x--
            y++ // 向右上移动
        }
    }
    ans := make([]byte, 0, n)
    for _, row := range mat {
        for _, ch := range row {
            if ch > 0 {
                ans = append(ans, ch)
            }
        }
    }
    return string(ans)
}