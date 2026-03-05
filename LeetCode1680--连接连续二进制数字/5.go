func concatenatedBinary(n int) (ans int) {
	const mod = 1_000_000_007
	for i := 1; i <= n; i++ {
		w := bits.Len(uint(i))
		ans = (ans<<w | i) % mod
	}
	return
}