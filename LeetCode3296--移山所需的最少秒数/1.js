const EPS = 1e-7;

var minNumberOfSeconds = function(mountainHeight, workerTimes) {
    const maxWorkerTimes = Math.max(...workerTimes);
    let l = 1;
    let r = maxWorkerTimes * mountainHeight * (mountainHeight + 1) / 2;
    let ans = 0;
    
    while (l <= r) {
        const mid = Math.floor((l + r) / 2);
        let cnt = 0;
        for (const t of workerTimes) {
            const work = Math.floor(mid / t);
            // 求最大的 k 满足 1+2+...+k <= work
            const k = Math.floor((-1.0 + Math.sqrt(1 + work * 8)) / 2 + EPS);
            cnt += k;
        }
        
        if (cnt >= mountainHeight) {
            ans = mid;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    return ans;
}