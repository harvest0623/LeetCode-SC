# 方法一：模拟
class Solution:
    def maxBottlesDrunk(self, numBottles: int, numExchange: int) -> int:
        ans = numBottles
        empty = numBottles
        while empty >= numExchange:
            ans += 1
            empty -= numExchange - 1
            numExchange += 1
        return ans

# 方法二：数学
class Solution:
    def maxBottlesDrunk(self, numBottles: int, numExchange: int) -> int:
        a = 1
        b = 2 * numExchange - 3
        c = -2 * numBottles
        delta = b * b - 4 * a * c
        t = math.ceil((-b + math.sqrt(delta)) / (2 * a))
        return numBottles + t - 1