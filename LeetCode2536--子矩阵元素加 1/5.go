func rangeAddQueries(n int, queries [][]int) [][]int {
	// 二维差分
	diff := make([][]int, n+2)
	for i := range diff {
		diff[i] = make([]int, n+2)
	}
	for _, q := range queries {
		r1, c1, r2, c2 := q[0], q[1], q[2], q[3]
		diff[r1+1][c1+1]++
		diff[r1+1][c2+2]--
		diff[r2+2][c1+1]--
		diff[r2+2][c2+2]++
	}

	// 原地计算 diff 的二维前缀和，然后填入答案
	ans := make([][]int, n)
	for i := range ans {
		ans[i] = make([]int, n)
		for j := range ans[i] {
			diff[i+1][j+1] += diff[i+1][j] + diff[i][j+1] - diff[i][j]
			ans[i][j] = diff[i+1][j+1]
		}
	}
	return ans
}