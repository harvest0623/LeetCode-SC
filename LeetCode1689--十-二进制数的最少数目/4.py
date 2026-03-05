class Solution:
    def minPartitions(self, n: str) -> int:
        res = 0
        for c in n:
            res = max(res, ord(c) - ord('0'))
        return res