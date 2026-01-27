var countTriples = function(n) {
    let res = 0;
    // 枚举 a 与 b
    for (let a = 1; a <= n; a++) {
        for (let b = 1; b <= n; b++) {
            // 判断是否符合要求
            let c = Math.floor(Math.sqrt(a * a + b * b + 1));
            if (c <= n && c * c === a * a + b * b) {
                res++;
            }
        }
    }
    return res;
};