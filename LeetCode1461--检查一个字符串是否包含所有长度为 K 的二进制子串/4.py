class Solution:
    def hasAllCodes(self, s: str, k: int) -> bool:
        if len(s) < (1 << k) + k - 1:
            return False
        exists = set(s[i:i+k] for i in range(len(s) - k + 1))
        return len(exists) == (1 << k)