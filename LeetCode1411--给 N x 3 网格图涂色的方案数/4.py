class Solution:
    def numOfWays(self, n: int) -> int:
        mod = 10**9 + 7
        preA, preB = 6, 6
        for i in range(2, n + 1):
            preA, preB = (2 * preA + 2 * preB) % mod, (2 * preA + 3 * preB) % mod
        return (preA + preB) % mod
