var smallestRepunitDivByK = function(k) {
    if (k % 2 === 0 || k % 5 === 0) {
        return -1;
    }
    let resid = 1 % k, len = 1;
    while (resid !== 0) {
        resid = (resid * 10 + 1) % k;
        len++;
    }
    return len;
};