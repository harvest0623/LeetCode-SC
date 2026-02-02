class Solution:
    def numberOfWays(self, corridor: str) -> int:
        mod = 10**9 + 7

        prev, cnt, ans = -1, 0, 1
        for i, ch in enumerate(corridor):
            if ch == "S":
                cnt += 1
                if cnt >= 3 and cnt % 2 == 1:
                    ans = ans * (i - prev) % mod
                prev = i
        if cnt < 2 or cnt % 2 == 1:
            ans = 0
        return ans