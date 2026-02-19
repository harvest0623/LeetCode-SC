func maximumTotalDamage(power []int) int64 {
	cnt := map[int]int{}
	for _, x := range power {
		cnt[x]++
	}
	n := len(cnt)
	a := make([]int, 0, n)
	for x := range cnt {
		a = append(a, x)
	}
	slices.Sort(a)
	f := make([]int, n+1)
	j := 0
	for i, x := range a {
		for a[j] < x-2 {
			j++
		}
		f[i+1] = max(f[i], f[j]+x*cnt[x])
	}
	return int64(f[n])
}