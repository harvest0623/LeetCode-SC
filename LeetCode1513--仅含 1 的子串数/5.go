func numSub(s string) (ans int) {
	const mod = 1e9 + 7
	last0 := -1
	for i, ch := range s {
		if ch == '0' {
			last0 = i  
		} else {
			ans += i - last0  
		}
	}
	return ans % mod
}