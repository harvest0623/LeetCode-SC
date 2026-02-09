var findSmallestInteger = function(nums, value) {
    const mp = new Array(value).fill(0);
    for (let x of nums) {
        const v = ((x % value) + value) % value;
        mp[v]++;
    }
    let mex = 0;
    while (mp[mex % value] > 0) {
        mp[mex % value]--;
        mex++;
    }
    return mex;
};