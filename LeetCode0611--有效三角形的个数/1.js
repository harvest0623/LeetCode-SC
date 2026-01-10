// 方法一:排序 + 二分查找
var triangleNumber = function(nums) {
    const n = nums.length;
    nums.sort((a, b) => a - b);
    let ans = 0;
    for (let i = 0; i < n; ++i) {
        for (let j = i + 1; j < n; ++j) {
            let left = j + 1, right = n - 1, k = j;
            while (left <= right) {
                const mid = Math.floor((left + right) / 2);
                if (nums[mid] < nums[i] + nums[j]) {
                    k = mid;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            ans += k - j;
        }
    }
    return ans;
};

// 方法二：排序 + 双指针
var triangleNumber = function(nums) {
    const n = nums.length;
    nums.sort((a, b) => a - b);
    let ans = 0;
    for (let i = 0; i < n; ++i) {
        let k = i;
        for (let j = i + 1; j < n; ++j) {
            while (k + 1 < n && nums[k + 1] < nums[i] + nums[j]) {
                ++k;
            }
            ans += Math.max(k - j, 0);
        }
    }
    return ans;
};