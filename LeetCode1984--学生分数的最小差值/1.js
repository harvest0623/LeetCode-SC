var minimumDifference = function(nums, k) {
    if (k === 1) return 0;
    nums.sort((a, b) => a - b);
    let ans = Infinity;
    for (let i = 0; i <= nums.length - k; i++) {
        ans = Math.min(ans, nums[i + k - 1] - nums[i]);
    }
    return ans;
};