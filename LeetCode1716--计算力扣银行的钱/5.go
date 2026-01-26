// 方法一：暴力计算
func totalMoney(n int) (ans int) {
    week, day := 1, 1
    for i := 0; i < n; i++ {
        ans += week + day - 1
        day++
        if day == 8 {
            day = 1
            week++
        }
    }
    return
}

// 方法二：等差数列求和
func totalMoney(n int) int {
	const d = 7
	w, r := n/d, n%d
	return (w*d*(w+d) + r*(w*2+r+1)) / 2
}