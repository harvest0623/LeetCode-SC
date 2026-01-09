class Solution:
    def findMaxForm(self, strs: List[str], m: int, n: int) -> int:
        length = len(strs)
        dp = [[[0] * (n + 1) for _ in range(m + 1)] for __ in range(length + 1)]

        def getZerosOnes(s: str) -> tuple[int, int]:
            zeros = ones = 0
            for c in s:
                if c == '0':
                    zeros += 1
                else:
                    ones += 1
            return zeros, ones

        for i in range(1, length + 1):
            zeros, ones = getZerosOnes(strs[i - 1])
            for j in range(m + 1):
                for k in range(n + 1):
                    dp[i][j][k] = dp[i - 1][j][k]
                    if j >= zeros and k >= ones:
                        dp[i][j][k] = max(dp[i][j][k], dp[i - 1][j - zeros][k - ones] + 1)

        return dp[length][m][n]