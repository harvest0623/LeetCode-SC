var getHappyString = function(n, k) {
    const chs = ['a', 'b', 'c'];
    let res = [];
    if (k > 3 * (1 << (n - 1))) {
        return res.join("");
    }
    for (let i = 0; i < n; i++) {
        let count = 1 << (n - i - 1);
        for (let c of chs) {
            if (res.length > 0 && res[res.length - 1] === c) {
                continue;
            }
            if (k <= count) {
                res.push(c);
                break;
            }
            k -= count;
        }
    }
    return res.join("");
};