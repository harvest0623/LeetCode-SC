class Solution:
    def minimumCost(self, source: str, target: str, original: List[str], changed: List[str], cost: List[int]) -> int:
        g = [[0x3f3f3f3f] * 26 for _ in range(26)]
        for i in range(26):
            g[i][i] = 0
        for i in range(len(original)):
            x = ord(original[i]) - 97
            y = ord(changed[i]) - 97
            g[x][y] = min(g[x][y], cost[i])
        for k in range(26):
            for i in range(26):
                if g[i][k] >= 0x3f3f3f3f:
                    continue
                for j in range(26):
                    g[i][j] = min(g[i][j], g[i][k] + g[k][j])
        ans = 0
        for i in range(len(source)):
            a = ord(source[i]) - 97
            b = ord(target[i]) - 97
            if g[a][b] >= 0x3f3f3f3f:
                return -1
            ans += g[a][b]
        return ans