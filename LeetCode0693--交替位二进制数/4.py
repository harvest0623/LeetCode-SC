# 方法一：模拟
class Solution:
    def hasAlternatingBits(self, n: int) -> bool:
        prev = 2
        while n:
            cur = n % 2
            if cur == prev:
                return False
            prev = cur
            n //= 2
        return True

# 方法二：位运算
class Solution:
    def hasAlternatingBits(self, n: int) -> bool:
        a = n ^ (n >> 1)
        return a & (a + 1) == 0