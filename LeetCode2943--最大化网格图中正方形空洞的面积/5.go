// 返回 a 排序后的最长连续递增子数组的长度
func f(a []int) (mx int) {
	slices.Sort(a)
	cnt := 0
	for i, x := range a {
		if i > 0 && x == a[i-1]+1 {
			cnt++
		} else {
			cnt = 1 // 重新计数
		}
		mx = max(mx, cnt)
	}
	return mx
}

func maximizeSquareHoleArea(_, _ int, hBars, vBars []int) int {
	side := min(f(hBars), f(vBars)) + 1
	return side * side
}