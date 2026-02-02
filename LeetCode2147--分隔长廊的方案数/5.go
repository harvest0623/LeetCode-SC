func numberOfWays(corridor string) int {
	const mod = 1e9 + 7
	prev, cnt, ans := -1, 0, 1
	for i, ch := range corridor {
		if ch == 'S' {
			cnt += 1
			if (cnt >= 3) && (cnt % 2 == 1) {
				ans = ans * (i - prev) % mod
			}
			prev = i
		}
	}
	if (cnt < 2) || (cnt % 2 == 1) {
		ans = 0
	}
	return ans
}