class Solution {
public:
    long long maxMatrixSum(vector<vector<int>>& matrix) {
        int n = matrix.size();
        int cnt = 0;   // 负数元素的数量
        long long total = 0;   // 所有元素的绝对值之和
        int mn = INT_MAX;   // 方阵元素的最小绝对值
        for (int i = 0; i < n; ++i){
            for (int j = 0; j < n; ++j){
                mn = min(mn, abs(matrix[i][j]));
                if (matrix[i][j] < 0){
                    ++cnt;
                }
                total += abs(matrix[i][j]);
            }
        }
        // 按照负数元素的数量的奇偶性讨论
        if (cnt % 2 == 0){
            return total;
        }
        else{
            return total - 2 * mn;
        }
    }
};