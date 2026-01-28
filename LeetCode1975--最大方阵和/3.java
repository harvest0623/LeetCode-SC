class Solution {
    public long maxMatrixSum(int[][] matrix) {
        int n = matrix.length;
        int cnt = 0;   // 负数元素的数量
        long total = 0;   // 所有元素的绝对值之和
        int mn = Integer.MAX_VALUE;   // 方阵元素的最小绝对值
        for (int i = 0; i < n; ++i){
            for (int j = 0; j < n; ++j){
                mn = Math.min(mn, Math.abs(matrix[i][j]));
                if (matrix[i][j] < 0){
                    ++cnt;
                }
                total += Math.abs(matrix[i][j]);
            }
        }
        // 按照负数元素的数量的奇偶性讨论
        if (cnt % 2 == 0){
            return total;
        } else {
            return total - 2 * mn;
        }
    }
}