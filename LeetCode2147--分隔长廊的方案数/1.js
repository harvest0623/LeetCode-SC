var numberOfWays = function(corridor) {
    const mod = 1000000007;
    const n = corridor.length;
    let prev = -1, cnt = 0, ans = 1;
    for (let i = 0; i < n; ++i) {
        if (corridor[i] === 'S') {
            ++cnt;
            if (cnt >= 3 && cnt % 2 === 1) {
                ans = Number((BigInt(ans) * BigInt(i - prev)) % BigInt(mod));
            }
            prev = i;
        }
    }
    if (cnt < 2 || cnt % 2 !== 0) {
        ans = 0;
    }
    return ans;
};