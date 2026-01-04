# 方法一
class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0 or x > 0 and x % 10 == 0:
            return False
        rev = 0
        while rev < x // 10:
            rev = rev * 10 + x % 10
            x //= 10
        return rev == x or rev == x // 10

# 方法二（字符串）
class Solution:
    def isPalindrome(self, x: int) -> bool:
        s = str(x)
        return s == s[::-1]  # s 是回文串