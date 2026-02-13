// 方法一：排序
var minimumCost = function(nums) {
    nums = [nums[0], ...nums.slice(1).sort((a, b) => a - b)];
    return nums.slice(0, 3).reduce((sum, num) => sum + num, 0);
};

// 方法二：维护最小值和次小值
var minimumCost = function(nums) {
    let n1 = Number.MAX_SAFE_INTEGER;
    let n2 = Number.MAX_SAFE_INTEGER;
    for (let i = 1; i < nums.length; i++) {
        const x = nums[i];
        if (x < n1) {
            n2 = n1;
            n1 = x;
        } else if (x < n2) {
            n2 = x;
        }
    }
    return nums[0] + n1 + n2;
};