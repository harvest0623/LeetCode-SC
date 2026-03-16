class Solution:
    def minFlips(self, s: str) -> int:
        # 示性函数
        I = lambda ch, x: int(ord(ch) - ord("0") == x)
        
        n = len(s)
        pre = [[0, 0] for _ in range(n)]
        # 注意 i=0 的边界情况
        for i in range(n):
            pre[i][0] = (0 if i == 0 else pre[i - 1][1]) + I(s[i], 1)
            pre[i][1] = (0 if i == 0 else pre[i - 1][0]) + I(s[i], 0)
        
        ans = min(pre[n - 1][0], pre[n - 1][1])
        if n % 2 == 1:
            # 如果 n 是奇数，还需要求出 suf
            suf = [[0, 0] for _ in range(n)]
            # 注意 i=n-1 的边界情况
            for i in range(n - 1, -1, -1):
                suf[i][0] = (0 if i == n - 1 else suf[i + 1][1]) + I(s[i], 1)
                suf[i][1] = (0 if i == n - 1 else suf[i + 1][0]) + I(s[i], 0)
            
            for i in range(n - 1):
                ans = min(ans, pre[i][0] + suf[i + 1][0])
                ans = min(ans, pre[i][1] + suf[i + 1][1])
        
        return ans