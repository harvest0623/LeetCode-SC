// 方法一：递归
func minimumOneBitOperations(n int) int {
    if n == 0 {
        return 0
    }
    x := bits.Len(uint(n)) - 1
    return (1 << (x + 1)) - 1 - minimumOneBitOperations(n - (1 << x))
}

// 方法二：迭代
func minimumOneBitOperations(n int) int {
    ans := 0
    sign := 1
    for i := 29; i >= 0; i-- {
        if (n & (1 << i)) != 0 {
            ans += sign * ((1 << (i + 1)) - 1)
            sign = -sign
        }
    }
    return ans
}

// 方法三：格雷码转二进制码
func minimumOneBitOperations(n int) int {
    ans := 0
    for n > 0 {
        ans ^= n
        n >>= 1
    }
    return ans
}