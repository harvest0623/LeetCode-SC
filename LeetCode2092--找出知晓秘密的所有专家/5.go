func findAllPeople(_ int, meetings [][]int, firstPerson int) []int {
	// 按照 time 从小到大排序
	slices.SortFunc(meetings, func(a, b []int) int { return a[2] - b[2] })

	// 一开始 0 和 firstPerson 都知道秘密
	haveSecret := map[int]bool{0: true, firstPerson: true}

	// 分组循环
	m := len(meetings)
	for i := 0; i < m; {
		// 在同一时间发生的会议，建图
		g := map[int][]int{}
		time := meetings[i][2]
		for ; i < m && meetings[i][2] == time; i++ {
			x, y := meetings[i][0], meetings[i][1]
			g[x] = append(g[x], y)
			g[y] = append(g[y], x)
		}

		// 每个连通块只要有一个人知道秘密，那么整个连通块的人都知道秘密
		vis := map[int]bool{} // 避免重复访问节点
		var dfs func(int)
		dfs = func(x int) {
			vis[x] = true
			haveSecret[x] = true
			for _, y := range g[x] {
				if !vis[y] {
					dfs(y)
				}
			}
		}
		for x := range g { 
			if haveSecret[x] && !vis[x] {
				dfs(x)
			}
		}
	}
	return slices.Collect(maps.Keys(haveSecret))
}