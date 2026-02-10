class Solution:
    def maxKDivisibleComponents(self, n: int, edges: List[List[int]], values: List[int], k: int) -> int:
        graph = [[] for _ in range(n)]
        for u, v in edges:
            graph[u].append(v)
            graph[v].append(u)
        result = 0

        def dfs(node, parent):
            nonlocal result
            total = values[node]
            for neighbor in graph[node]:
                if neighbor != parent:
                    total += dfs(neighbor, node)
            if total % k == 0:
                result += 1
            return total

        dfs(0, -1)
        return result