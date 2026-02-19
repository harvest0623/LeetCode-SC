var countBinarySubstrings = function(s) {
    let pre = 0, cnt = 1, ans = 0;
    for (let i = 0; i < s.length - 1; i++) {
        if (s[i] === s[i + 1]) {
            cnt++;
        } else {
            ans += Math.min(pre, cnt);
            pre = cnt;
            cnt = 1;
        }
    }
    ans += Math.min(pre, cnt);
    return ans;
};