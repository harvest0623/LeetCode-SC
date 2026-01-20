var sumFourDivisors = function (nums) {
    let res = 0;
    for (const n of nums) {
        let cnt = 0, sum = 0;
        for (let i = 1; i * i <= n; ++i) {
            if (n % i == 0) {
                ++cnt;
                sum += i;
                if (i * i !== n) {
                    ++cnt;
                    sum += n / i;
                }
            }
        }
        if (cnt === 4) res += sum;
    }
    return res;
};