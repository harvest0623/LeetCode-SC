class Solution {
public:
    int maxSideLength(vector<vector<int>>& mat, int threshold) {
        int m = mat.size(), n = mat[0].size();
        vector<vector<int>> prefix(m + 1, vector<int>(n + 1, 0));
        
        // 计算二维前缀和
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                prefix[i][j] = prefix[i-1][j] + prefix[i][j-1] - prefix[i-1][j-1] + mat[i-1][j-1];
            }
        }
        
        // 二分查找最大边长
        int left = 0, right = min(m, n);
        int ans = 0;

        while (left <= right) {
            int mid = left + (right - left) / 2;
            bool found = false;
            
            // 检查所有边长为mid的正方形
            for (int i = 1; i + mid <= m + 1; i++) {
                for (int j = 1; j + mid <= n + 1; j++) {
                    int sum = prefix[i+mid-1][j+mid-1] - prefix[i-1][j+mid-1] 
                            - prefix[i+mid-1][j-1] + prefix[i-1][j-1];
                    if (sum <= threshold) {
                        found = true;
                        break;
                    }
                }
                if (found) break;
            }
            if (found) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return ans;
    }
};