// 方法一：遍历 (O(m × n))
func countNegatives(grid [][]int) int {
	cnt := 0
	for _, row := range grid {
		for _, val := range row {
			if val < 0 {
				cnt++
			}
		}
	}
	return cnt
}

// 方法二：有序性 (O(m + n))
func countNegatives(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	r, c := 0, n-1
	cnt := 0
	for r < m && c >= 0 {
		if grid[r][c] < 0 {
			cnt += m - r
			c--
		} else {
			r++
		}
	}
	return cnt
}