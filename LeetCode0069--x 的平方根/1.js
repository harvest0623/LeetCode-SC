const SQRT_MAX = Math.floor(Math.sqrt(2 ** 31 - 1));
var mySqrt = function (x) {
    let left = 0, right = Math.min(x, SQRT_MAX) + 1;
    while (left + 1 < right) {
        let m = Math.floor((left + right) / 2);
        if (m * m <= x) {
            left = m;
        } else {
            right = m;
        }
    }
    return left;
};