var hasAllCodes = function(s, k) {
    if (s.length < (1 << k) + k - 1) {
        return false;
    }
    const exists = new Set();
    for (let i = 0; i + k <= s.length; ++i) {
        exists.add(s.substring(i, i + k));
    }
    return exists.size === (1 << k);
};