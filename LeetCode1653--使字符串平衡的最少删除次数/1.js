var minimumDeletions = function(s) {
    let ans = 0;
    let cntA = 0, cnt = 0;
    for (let c of s) {
        cnt += c === 'a' ? -1 : 1;
        cntA += c === 'a';
        ans = Math.min(ans, cnt);
    }
    return cntA + ans;
};