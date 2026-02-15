func minimumBoxes(apple, capacity []int) int {
	s := 0
	for _, x := range apple {
		s += x 
	}
	slices.SortFunc(capacity, func(a, b int) int { return b - a }) // 先装大箱子，再装小箱子
	for i, c := range capacity {
		s -= c
		if s <= 0 { 
			return i + 1 
		}
	}
	return -1
}