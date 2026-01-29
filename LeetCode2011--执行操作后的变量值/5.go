func finalValueAfterOperations(operations []string) (ans int) {
	for _, s := range operations {
		if s[1] == '+' {
			ans++
		} else {
			ans--
		}
	}
	return
}