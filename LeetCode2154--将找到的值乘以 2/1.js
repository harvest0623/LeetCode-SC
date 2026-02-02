// 方法一：排序
var findFinalValue = function(nums, original) {
    nums.sort((a, b) => a - b);
    for (const num of nums) {
        if (original === num) {
            original *= 2;
        }
    }
    return original;
};

// 方法二：哈希表
var findFinalValue = function(nums, original) {
    const set = new Set(nums);
    while (set.has(original)) {
        original *= 2;
    }
    return original;
};