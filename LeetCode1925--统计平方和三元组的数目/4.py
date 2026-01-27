from math import sqrt

class Solution:
    def countTriples(self, n: int) -> int:
        res = 0
        # 枚举 a 与 b
        for a in range(1, n + 1):
            for b in range(1, n + 1):
                # 判断是否符合要求
                c = int(sqrt(a ** 2 + b ** 2 + 1))
                if c <= n and c ** 2 == a ** 2 + b ** 2:
                    res += 1
        return res