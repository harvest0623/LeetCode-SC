// 方法一：正序遍历
var isOneBitCharacter = function(bits) {
    let i = 0, n = bits.length;
    while (i < n - 1) {
        i += bits[i] + 1;
    }
    return i === n - 1;
};

// 方法二：倒序遍历
var isOneBitCharacter = function(bits) {
    let i = bits.length - 2;
    while (i >= 0 && bits[i]) {
        i--;
    }
    return (bits.length - i) % 2 === 0;
};