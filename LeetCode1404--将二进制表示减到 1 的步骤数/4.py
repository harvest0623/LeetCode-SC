class Solution:
    def numSteps(self, s: str) -> int:
        ans = len(s) - 1
        i = s.rfind('1')
        if i > 0:
            ans += s[1:i].count('0') + 2
        return ans