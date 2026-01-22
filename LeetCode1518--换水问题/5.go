// 方法一：模拟
func numWaterBottles(numBottles int, numExchange int) int {
	bottle := numBottles
	ans := numBottles
	for bottle >= numExchange {
		bottle -= numExchange
		ans++
		bottle++
	}
	return ans
}

// 方法二：数学
func numWaterBottles(numBottles int, numExchange int) int {
    if numBottles < numExchange {
        return numBottles
    }
    return (numBottles - numExchange) / (numExchange - 1) + 1 + numBottles
}