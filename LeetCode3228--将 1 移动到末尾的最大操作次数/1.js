var maxOperations = function(s) {
    let countOne = 0;
    let ans = 0;
    let i = 0;
    while (i < s.length) {
        if (s[i] === '0') {
            while (i + 1 < s.length && s[i + 1] === '0') {
                i++;
            }
            ans += countOne;
        } else {
            countOne++;
        }
        i++;
    }
    return ans;
};