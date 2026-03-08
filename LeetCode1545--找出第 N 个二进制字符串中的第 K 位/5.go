func findKthBit(n, k int) byte {
	if n == 1 {
		return '0'
	}
	if k == 1<<(n-1) {
		return '1'
	}
	if k < 1<<(n-1) {
		return findKthBit(n-1, k)
	}
	return findKthBit(n-1, 1<<n-k) ^ 1
}