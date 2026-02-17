// 方法一：模拟
var maxBottlesDrunk = function(numBottles, numExchange) {
    let ans = numBottles;
    let empty = numBottles;
    while (empty >= numExchange) {
        ans++;
        empty -= numExchange - 1;
        numExchange++;
    }
    return ans;
};

// 方法二：数学
var maxBottlesDrunk = function(numBottles, numExchange) {
    let a = 1;
    let b = 2 * numExchange - 3;
    let c = -2 * numBottles;
    let delta = b * b - 4 * a * c;
    let t = Math.ceil((-b + Math.sqrt(delta)) / (2 * a));
    return numBottles + t - 1;
};