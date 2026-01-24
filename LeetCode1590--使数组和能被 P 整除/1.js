var minSubarray = function(nums, p) {
    let x = 0;
    for (const num of nums) {
        x = (x + num) % p;
    }
    if (x === 0) {
        return 0;
    }
    const index = new Map();
    let y = 0, res = nums.length;
    for (let i = 0; i < nums.length; i++) {
        index.set(y, i); // f[i] mod p = y，因此哈希表记录 y 对应的下标为 i
        y = (y + nums[i]) % p;
        if (index.has((y - x + p) % p)) {
            res = Math.min(res, i - index.get((y - x + p) % p) + 1);
        }
    }
    return res === nums.length ? -1 : res;
};