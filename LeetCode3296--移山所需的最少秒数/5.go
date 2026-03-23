func minNumberOfSeconds(mountainHeight int, workerTimes []int) int64 {
	h := make(hp, len(workerTimes))
	for i, t := range workerTimes {
		h[i] = worker{t, t, t}
	}
	heap.Init(&h)

	ans := 0
	for range mountainHeight {
		ans = h[0].total // 最后一个出堆的 total 即为答案
		h[0].cur += h[0].base
		h[0].total += h[0].cur
		heap.Fix(&h, 0)
	}
	return int64(ans)
}

// 工作后总用时，当前工作（山高度降低 1）用时，workerTimes[i]
type worker struct{ total, cur, base int }
type hp []worker
func (h hp) Len() int           { return len(h) }
func (h hp) Less(i, j int) bool { return h[i].total < h[j].total }
func (h hp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (hp) Push(any)             {}
func (hp) Pop() (_ any)         { return }