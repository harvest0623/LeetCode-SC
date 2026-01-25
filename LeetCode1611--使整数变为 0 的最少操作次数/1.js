// 方法一：递归
var minimumOneBitOperations = function(n) {
    if (n === 0) {
        return 0;
    }
    const x = Math.floor(Math.log2(n));
    return (1 << (x + 1)) - 1 - minimumOneBitOperations(n - (1 << x));
};

// 方法二：迭代
var minimumOneBitOperations = function(n) {
    let ans = 0;
    let sign = 1;
    for (let i = 29; i >= 0; --i) {
        if (n & (1 << i)) {
            ans += sign * ((1 << (i + 1)) - 1);
            sign = -sign;
        }
    }
    return ans;
};

// 方法三：格雷码转二进制码
var minimumOneBitOperations = function(n) {
    let ans = 0;
    while (n) {
        ans ^= n;
        n >>= 1;
    }
    return ans;
};