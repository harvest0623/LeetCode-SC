func maxFrequencyElements(nums []int) (ans int) {
	cnt := map[int]int{}
	maxCnt := 0
	for _, x := range nums {
		cnt[x]++
		c := cnt[x]
		if c > maxCnt {
			maxCnt = c
			ans = c
		} else if c == maxCnt {
			ans += c
		}
	}
	return
}