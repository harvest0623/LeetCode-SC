// 方法一：模拟
var hasAlternatingBits = function(n) {
    let prev = 2;
    while (n !== 0) {
        const cur = n % 2;
        if (cur === prev) {
            return false;
        }
        prev = cur;
        n = Math.floor(n / 2);
    }
    return true;
};

// 方法二：位运算
var hasAlternatingBits = function(n) {
    const a = n ^ (n >> 1);
    return (a & (a + 1)) === 0;
};