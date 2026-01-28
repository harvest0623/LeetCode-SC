var maxMatrixSum = function(matrix) {
    const n = matrix.length;
    let cnt = 0;   // 负数元素的数量
    let total = 0;   // 所有元素的绝对值之和
    let mn = Number.MAX_SAFE_INTEGER;   // 方阵元素的最小绝对值
    for (let i = 0; i < n; i++) {
        for (let j = 0; j < n; j++) {
            const absVal = Math.abs(matrix[i][j]);
            mn = Math.min(mn, absVal);
            if (matrix[i][j] < 0) {
                cnt++;
            }
            total += absVal;
        }
    }
    // 按照负数元素的数量的奇偶性讨论
    if (cnt % 2 === 0) {
        return total;
    } else {
        return total - 2 * mn;
    }
};