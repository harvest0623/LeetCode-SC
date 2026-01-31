func maxTwoEvents(events [][]int) (ans int) {
	// 按照结束时间排序
	slices.SortFunc(events, func(a, b []int) int { return a[1] - b[1] })

	// 从栈底到栈顶，结束时间递增，价值递增
	type pair struct{ endTime, value int }
	st := []pair{{}} // 栈底哨兵
	for _, e := range events {
		startTime, value := e[0], e[2]
		// 二分查找最后一个结束时间 < startTime 的活动
		i := sort.Search(len(st), func(i int) bool { return st[i].endTime >= startTime }) - 1
		ans = max(ans, st[i].value+value)
		// 遇到比栈顶更大的价值，入栈
		if value > st[len(st)-1].value {
			st = append(st, pair{e[1], value})
		}
	}
	return
}