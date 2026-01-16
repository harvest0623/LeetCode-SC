var minDeletionSize = function(strs) {
    const n = strs[0].length;
    const dp = new Array(n).fill(1);
    for (let i = n - 2; i >= 0; i--) {
        for (let j = i + 1; j < n; j++) {
            let valid = true;
            for (const row of strs) {
                if (row[i] > row[j]) {
                    valid = false;
                    break;
                }
            }
            if (valid) {
                dp[i] = Math.max(dp[i], 1 + dp[j]);
            }
        }
    }
    return n - Math.max(...dp);
};