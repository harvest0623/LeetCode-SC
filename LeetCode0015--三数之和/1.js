var threeSum = function (nums) {
    nums.sort((a, b) => a - b);
    const n = nums.length;
    const ans = [];
    for (let i = 0; i < n - 2; i++) {
        const x = nums[i];
        if (i > 0 && x === nums[i - 1]) continue;
        if (x + nums[i + 1] + nums[i + 2] > 0) break;
        if (x + nums[n - 2] + nums[n - 1] < 0) continue;
        let j = i + 1, k = n - 1;
        while (j < k) {
            const s = x + nums[j] + nums[k];
            if (s > 0) {
                k--;
            } else if (s < 0) {
                j++;
            } else { 
                ans.push([x, nums[j], nums[k]]);
                for (j++; j < k && nums[j] === nums[j - 1]; j++);
                for (k--; k > j && nums[k] === nums[k + 1]; k--);
            }
        }
    }
    return ans;
};