// 方法一：动态规划
var minimumTotal = function(triangle) {
    const n = triangle.length;
    const f = Array(n).fill(0).map(() => Array(n).fill(0));
    f[0][0] = triangle[0][0];
    for (let i = 1; i < n; ++i) {
        f[i][0] = f[i - 1][0] + triangle[i][0];
        for (let j = 1; j < i; ++j) {
            f[i][j] = Math.min(f[i - 1][j - 1], f[i - 1][j]) + triangle[i][j];
        }
        f[i][i] = f[i - 1][i - 1] + triangle[i][i];
    }
    return Math.min(...f[n - 1]);
};

// 方法二：动态规划 + 空间优化
var minimumTotal = function(triangle) {
    const n = triangle.length;
    const f = new Array(n).fill(0);
    f[0] = triangle[0][0];
    
    for (let i = 1; i < n; ++i) {
        f[i] = f[i - 1] + triangle[i][i];
        for (let j = i - 1; j > 0; --j) {
            f[j] = Math.min(f[j - 1], f[j]) + triangle[i][j];
        }
        f[0] += triangle[i][0];
    }
    
    return Math.min(...f);
};