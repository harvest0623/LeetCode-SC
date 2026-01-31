var maxTwoEvents = function(events) {
    // 按 endTime 排序
    events.sort((a, b) => a[1] - b[1]);
    const n = events.length;
    const maxValue = new Array(n);
    maxValue[0] = events[0][2];
    // 前缀最大 value
    for (let i = 1; i < n; i++) {
        maxValue[i] = Math.max(maxValue[i - 1], events[i][2]);
    }
    let ans = 0;
    for (let i = 0; i < n; i++) {
        let curVal = events[i][2];
        let l = 0, r = i - 1, pos = -1;
        // 二分找 end < start
        while (l <= r) {
            const mid = Math.floor((l + r) / 2);
            if (events[mid][1] < events[i][0]) {
                pos = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        if (pos !== -1) {
            curVal += maxValue[pos];
        }
        ans = Math.max(ans, curVal);
    }
    return ans;
};