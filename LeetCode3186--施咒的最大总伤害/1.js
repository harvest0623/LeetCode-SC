var maximumTotalDamage = function (power) {
    let count = new Map();
    for (let p of power) {
        count.set(p, (count.get(p) || 0) + 1);
    }
    let vec = [[-1000000000, 0]];
    let keys = Array.from(count.keys()).sort((a, b) => a - b);
    for (let k of keys) {
        vec.push([k, count.get(k)]);
    }
    let n = vec.length;
    let f = Array(n).fill(0);
    let mx = 0, ans = 0, j = 1;
    for (let i = 1; i < n; i++) {
        while (j < i && vec[j][0] < vec[i][0] - 2) {
            mx = Math.max(mx, f[j]);
            j++;
        }
        f[i] = mx + vec[i][0] * vec[i][1];
        ans = Math.max(ans, f[i]);
    }
    return ans;
};