// 方法一：正序遍历
func isOneBitCharacter(bits []int) bool {
    n, i := len(bits), 0
    for i < n - 1 {
        i += bits[i] + 1
    }
    return i == n - 1
}

// 方法二：倒序遍历
func isOneBitCharacter(bits []int) bool {
    n, i := len(bits), n - 2
    for i >= 0 && bits[i] == 1 {
        i--
    }
    return (n - i) % 2 == 0
}