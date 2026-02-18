// 方法一：模拟
func hasAlternatingBits(n int) bool {
    for pre := 2; n != 0; n /= 2 {
        cur := n % 2
        if cur == pre {
            return false
        }
        pre = cur
    }
    return true
}

// 方法二：位运算
func hasAlternatingBits(n int) bool {
    a := n ^ n>>1
    return a&(a+1) == 0
}