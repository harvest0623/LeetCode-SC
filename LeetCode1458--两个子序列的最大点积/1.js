var maxDotProduct = function(nums1, nums2) {
    const m = nums1.length;
    const n = nums2.length;
    const f = Array.from({ length: m }, () => new Array(n).fill(0));
    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j) {
            const xij = nums1[i] * nums2[j];
            f[i][j] = xij;
            if (i > 0) {
                f[i][j] = Math.max(f[i][j], f[i - 1][j]);
            }
            if (j > 0) {
                f[i][j] = Math.max(f[i][j], f[i][j - 1]);
            }
            if (i > 0 && j > 0) {
                f[i][j] = Math.max(f[i][j], f[i - 1][j - 1] + xij);
            }
        }
    }
    return f[m - 1][n - 1];
};