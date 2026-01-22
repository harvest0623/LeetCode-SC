# 方法一：模拟
def numWaterBottles(numBottles, numExchange):
    bottle = numBottles
    ans = numBottles
    while bottle >= numExchange:
        bottle -= numExchange
        ans += 1
        bottle += 1
    return ans

# 方法二：数学
class Solution:
    def numWaterBottles(self, numBottles: int, numExchange: int) -> int:
        return (numBottles - numExchange) // (numExchange - 1) + 1 + numBottles if numBottles >= numExchange else numBottles