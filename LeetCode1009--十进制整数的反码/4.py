class Solution:
    def bitwiseComplement(self, n: int) -> int:
        highbit = 0
        for i in range(1, 30 + 1):
            if n >= (1 << i):
                highbit = i
            else:
                break
        
        mask = (1 << (highbit + 1)) - 1
        return n ^ mask