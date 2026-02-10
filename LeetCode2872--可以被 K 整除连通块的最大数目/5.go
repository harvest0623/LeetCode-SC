func maxKDivisibleComponents(n int, edges [][]int, values []int, k int) int {
	graph := make([][]int, n)
	for _, edge := range edges {
		u, v := edge[0], edge[1]
		graph[u] = append(graph[u], v)
		graph[v] = append(graph[v], u)
	}
	result := 0
	var dfs func(int, int) int64
	dfs = func(node, parent int) int64 {
		sum := int64(values[node])
		for _, neighbor := range graph[node] {
			if neighbor != parent {
				sum += dfs(neighbor, node)
			}
		}
		if sum%int64(k) == 0 {
			result++
		}
		return sum
	}
	dfs(0, -1)
	return result
}