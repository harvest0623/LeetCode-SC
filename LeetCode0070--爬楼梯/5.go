// 方法一：动态规划
func climbStairs(n int) int {
    p, q, r := 0, 0, 1
    for i := 1; i <= n; i++ {
        p = q
        q = r
        r = p + q
    }
    return r
}

// 方法二：矩阵快速幂
type matrix [2][2]int

func mul(a, b matrix) (c matrix) {
    for i := 0; i < 2; i++ {
        for j := 0; j < 2; j++ {
            c[i][j] = a[i][0]*b[0][j] + a[i][1]*b[1][j]
        }
    }
    return c
}

func pow(a matrix, n int) matrix {
    res := matrix{{1, 0}, {0, 1}}
    for ; n > 0; n >>= 1 {
        if n&1 == 1 {
            res = mul(res, a)
        }
        a = mul(a, a)
    }
    return res
}

func climbStairs(n int) int {
    res := pow(matrix{{1, 1}, {1, 0}}, n)
    return res[0][0]
}

// 方法三：通项公式
func climbStairs(n int) int {
    sqrt5 := math.Sqrt(5)
    pow1 := math.Pow((1+sqrt5)/2, float64(n+1))
    pow2 := math.Pow((1-sqrt5)/2, float64(n+1))
    return int(math.Round((pow1 - pow2) / sqrt5))
}