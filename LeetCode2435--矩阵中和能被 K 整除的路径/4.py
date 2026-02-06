class Solution:
    def numberOfPaths(self, grid: List[List[int]], k: int) -> int:
        MOD = 1_000_000_007

        @cache  # 缓存装饰器，避免重复计算 dfs（一行代码实现记忆化）
        def dfs(i: int, j: int, s: int) -> int:
            if i < 0 or j < 0:  # 出界
                return 0
            pre_s = (s - grid[i][j]) % k
            if i == 0 and j == 0:  # 起点
                return 1 if pre_s == 0 else 0  # pre_s == 0 说明 s == grid[i][j] % k
            return (dfs(i - 1, j, pre_s) + dfs(i, j - 1, pre_s)) % MOD

        ans = dfs(len(grid) - 1, len(grid[0]) - 1, 0)
        dfs.cache_clear()  # 避免超出内存限制
        return ans