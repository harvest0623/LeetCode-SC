var findDifferentBinaryString = function(nums) {
    const n = nums.length;
    // 预处理对应整数的哈希集合
    const vals = new Set();
    for (const num of nums) {
        vals.add(parseInt(num, 2));
    }
    // 寻找第一个不在哈希集合中的整数
    let val = 0;
    while (vals.has(val)) {
        ++val;
    }
    // 将整数转化为二进制字符串返回
    let binary = val.toString(2);
    // 补齐前导0
    while (binary.length < n) {
        binary = '0' + binary;
    }
    return binary;
};