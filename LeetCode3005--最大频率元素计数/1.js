var maxFrequencyElements = function(nums) {
    const cnt = new Map();
    let ans = 0, maxCnt = 0;
    for (const x of nums) {
        const c = (cnt.get(x) ?? 0) + 1;
        cnt.set(x, c);
        if (c > maxCnt) {
            ans = maxCnt = c;
        } else if (c === maxCnt) {
            ans += c;
        }
    }
    return ans;
};