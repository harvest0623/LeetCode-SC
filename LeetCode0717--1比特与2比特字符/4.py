# 方法一：正序遍历
class Solution:
    def isOneBitCharacter(self, bits: List[int]) -> bool:
        n, i = len(bits), 0
        while i < n - 1:
            i += bits[i] + 1
        return i == n - 1

# 方法二：倒序遍历
class Solution:
    def isOneBitCharacter(self, bits: List[int]) -> bool:
        n, i = len(bits), n - 2
        while i >= 0 and bits[i]:
            i -= 1
        return (n - i) % 2 == 0
