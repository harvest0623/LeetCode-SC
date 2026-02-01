var maxRunTime = function(n, batteries) {
    let sum = batteries.reduce((acc, val) => acc + val, 0);
    let left = 0, right = Math.floor(sum / n), ans = 0;
    while (left <= right) {
        let mid = Math.floor((left + right) / 2);
        let total = 0;
        for (let cap of batteries) {
            total += Math.min(cap, mid);
        }
        if (total >= n * mid) {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return ans;
};