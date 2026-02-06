var bestClosingTime = function(customers) {
    const n = customers.length;
    let suf = 0;
    let pre = 0;
    let minCost = 0;
    let res = 0;
    for (let i = 0; i <= n; i++) {
        if (minCost > suf + pre) {
            minCost = suf + pre;
            res = i;
        }
        if (i < n && customers[i] === 'N') {
            pre++;
        } else if (i < n) {
            suf--;
        }
    }
    return res;
};