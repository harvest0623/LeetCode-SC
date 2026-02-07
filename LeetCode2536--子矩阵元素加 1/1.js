var rangeAddQueries = function(n, queries) {
    // 二维差分
    const diff = Array.from({ length: n + 2 }, () => Array(n + 2).fill(0));
    for (const [r1, c1, r2, c2] of queries) {
        diff[r1 + 1][c1 + 1]++;
        diff[r1 + 1][c2 + 2]--;
        diff[r2 + 2][c1 + 1]--;
        diff[r2 + 2][c2 + 2]++;
    }

    // 原地计算 diff 的二维前缀和，然后填入答案
    const ans = Array.from({ length: n }, () => Array(n).fill(0));
    for (let i = 0; i < n; i++) {
        for (let j = 0; j < n; j++) {
            diff[i + 1][j + 1] += diff[i + 1][j] + diff[i][j + 1] - diff[i][j];
            ans[i][j] = diff[i + 1][j + 1];
        }
    }
    return ans;
};