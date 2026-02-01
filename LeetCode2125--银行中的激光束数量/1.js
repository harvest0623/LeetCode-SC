var numberOfBeams = function(bank) {
    let last = 0, ans = 0;
    for (const line of bank) {
        const cnt = (line.match(/1/g) || []).length;
        if (cnt !== 0) {
            ans += last * cnt;
            last = cnt;
        }
    }
    return ans;
};