class Solution:
    def countBinarySubstrings(self, s: str) -> int:
        pre = 0
        cnt = 1
        ans = 0
        for i in range(len(s) - 1):
            if s[i] == s[i + 1]:
                cnt += 1
            else:
                ans += min(pre, cnt)
                pre = cnt
                cnt = 1
        ans += min(pre, cnt)
        return ans