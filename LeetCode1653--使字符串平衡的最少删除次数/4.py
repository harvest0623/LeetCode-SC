class Solution:
    def minimumDeletions(self, s: str) -> int:
        ans = 0
        cntA = 0
        cnt = 0
        for ch in s:
            cnt += -1 if ch == 'a' else 1
            cntA += 1 if ch == 'a' else 0
            ans = min(ans, cnt)
        return cntA + ans