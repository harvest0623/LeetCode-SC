var minPartitions = function(n) {
    let res = 0;
    for (const c of n) {
        res = Math.max(res, c.charCodeAt(0) - '0'.charCodeAt(0));
    }
    return res;
};