class Solution:
    def swimInWater(self, grid: List[List[int]]) -> int:
        n = len(grid)
        def check(mx: int) -> bool:
            vis = set()
            def dfs(i: int, j: int) -> bool:
                if i == j == n - 1:  # 到达终点
                    return True
                vis.add((i, j))  # 标记访问过，避免重复访问
                for x, y in (i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1):  # 按照这个顺序访问邻居，代码跑得快（和数据有关系）
                    if 0 <= x < n and 0 <= y < n and grid[x][y] <= mx and (x, y) not in vis and dfs(x, y):
                        return True
                return False

            return dfs(0, 0)

        left = max(grid[0][0], grid[-1][-1]) - 1
        right = n * n - 1
        while left + 1 < right: 
            mid = (left + right) // 2
            if check(mid):
                right = mid
            else:
                left = mid
        return right