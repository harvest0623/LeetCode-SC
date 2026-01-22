// 方法一：模拟
var numWaterBottles = function(numBottles, numExchange) {
    let bottle = numBottles, ans = numBottles;
    while (bottle >= numExchange) {
        bottle -= numExchange;
        ++ans;
        ++bottle;
    }
    return ans;
};

// 方法二：数学
var numWaterBottles = function(numBottles, numExchange) {
    return numBottles >= numExchange ? Math.floor((numBottles - numExchange) / (numExchange - 1)) + 1 + numBottles : numBottles;
};