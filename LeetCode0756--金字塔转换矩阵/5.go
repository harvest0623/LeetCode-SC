func pyramidTransition(bottom string, allowed []string) bool {
	groups := [6][6][]byte{}
	for _, s := range allowed {
		a, b := s[0]-'A', s[1]-'A'
		groups[a][b] = append(groups[a][b], s[2])
	}
	n := len(bottom)
	pyramid := make([][]byte, n)
	for i := range n - 1 {
		pyramid[i] = make([]byte, i+1)
	}
	pyramid[n-1] = []byte(bottom)
	vis := map[string]struct{}{} 
	var dfs func(int, int) bool
	dfs = func(i, j int) bool {
		if i < 0 {
			return true
		}
		if j == i+1 {
			row := string(pyramid[i])
			if _, ok := vis[row]; ok { 
				return false 
			}
			vis[row] = struct{}{}
			return dfs(i-1, 0)
		}
		for _, top := range groups[pyramid[i+1][j]-'A'][pyramid[i+1][j+1]-'A'] {
			pyramid[i][j] = top
			if dfs(i, j+1) {
				return true
			}
		}
		return false
	}
	return dfs(n-2, 0)
}