// 方法一：暴力计算
var totalMoney = function(n) {
    let week = 1, day = 1;
    let res = 0;
    for (let i = 0; i < n; ++i) {
        res += week + day - 1;
        ++day;
        if (day === 8) {
            day = 1;
            ++week;
        }
    }
    return res;
};

// 方法二：等差数列求和
var totalMoney = function(n) {
    const D = 7;
    const w = Math.floor(n / D), r = n % D;
    return (w * D * (w + D) + r * (w * 2 + r + 1)) / 2;
};