class Solution:
    def myPow(self, x: float, n: int) -> float:
        if n < 0:
            return self.myPow(1 / x, -n)
        if n == 0:
            return 1.0
        return self.myPow(x, n // 2) ** 2 * (x if n % 2 else 1.0)