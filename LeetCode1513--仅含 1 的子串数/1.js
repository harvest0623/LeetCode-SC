var numSub = function(s) {
    const MOD = 1e9 + 7;
    let ans = 0, last0 = -1;
    for (let i = 0; i < s.length; i++) {
        if (s[i] === '0') {
            last0 = i; // 记录上个 0 的位置
        } else {
            ans += i - last0; // 右端点为 i 的全 1 子串个数
        }
    }
    return ans % MOD;
};