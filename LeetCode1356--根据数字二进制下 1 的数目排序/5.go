func sortByBits(arr []int) []int {
	slices.SortFunc(arr, func(a, b int) int {
		return cmp.Or(bits.OnesCount(uint(a))-bits.OnesCount(uint(b)), a-b)
	})
	return arr
}