var minPairSum = function(nums) {
    nums.sort((a, b) => a - b);
    let maxSum = 0;
    const n = nums.length;
    for (let i = 0; i < n / 2; i++) {
        maxSum = Math.max(maxSum, nums[i] + nums[n - 1 - i]);
    }
    return maxSum;
};