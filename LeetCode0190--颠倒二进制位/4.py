# 方法一：逐位颠倒
class Solution:
    def reverseBits(self, n: int) -> int:
        rev = 0
        for i in range(32):
            if n == 0:
                break
            rev |= (n & 1) << (31 - i)
            n >>= 1
        return rev

# 方法二：位运算分治
class Solution:
    def reverseBits(self, n: int) -> int:
        M1 = 0x55555555  # 01010101010101010101010101010101
        M2 = 0x33333333  # 00110011001100110011001100110011
        M4 = 0x0f0f0f0f  # 00001111000011110000111100001111
        M8 = 0x00ff00ff  # 00000000111111110000000011111111
        
        n = n & 0xFFFFFFFF
        n = (n >> 1 & M1) | ((n & M1) << 1)
        n = (n >> 2 & M2) | ((n & M2) << 2)
        n = (n >> 4 & M4) | ((n & M4) << 4)
        n = (n >> 8 & M8) | ((n & M8) << 8)
        return ((n >> 16) | (n << 16)) & 0xFFFFFFFF