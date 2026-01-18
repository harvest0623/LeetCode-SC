class Solution {
    public int maxSideLength(int[][] mat, int threshold) {
        int m = mat.length, n = mat[0].length;
        int[][] prefix = new int[m + 1][n + 1];
        
        // 计算二维前缀和
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                prefix[i][j] = prefix[i-1][j] + prefix[i][j-1] - prefix[i-1][j-1] + mat[i-1][j-1];
            }
        }
        
        // 二分查找
        int left = 0, right = Math.min(m, n);
        int ans = 0;
        
        while (left <= right) {
            int mid = left + (right - left) / 2;
            boolean found = false;
            
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
}