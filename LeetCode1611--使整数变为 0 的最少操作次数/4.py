# 方法一：递归
class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        if n == 0:
            return 0
        x = len(bin(n)) - 3
        return (1 << (x + 1)) - 1 - self.minimumOneBitOperations(n - (1 << x))

# 方法二：迭代class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        ans = 0
        sign = 1
        for i in range(29, -1, -1):
            if n & (1 << i):
                ans += sign * ((1 << (i + 1)) - 1)
                sign = -sign
        return ans

# 方法三：格雷码转二进制码
class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        ans = 0
        while n:
            ans ^= n
            n >>= 1
        return ans