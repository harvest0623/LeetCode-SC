class Solution:
    def findKthBit(self, n: int, k: int) -> str:
        if n == 1:
            return '0'
        if k == 1 << (n - 1):
            return '1'
        if k < 1 << (n - 1):
            return self.findKthBit(n - 1, k)
        res = self.findKthBit(n - 1, (1 << n) - k)
        return '0' if res == '1' else '1'