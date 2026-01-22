class Solution:
    def numSub(self, s: str) -> int:
        MOD = 1e9 + 7
        ans = 0
        last0 = -1
        for i, ch in enumerate(s):
            if ch == '0':
                last0 = i  
            else:
                ans += i - last0  
        return ans % MOD