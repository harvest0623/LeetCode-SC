public class Solution {
    public bool FindRotation(int[][] mat, int[][] target) {
        int n = mat.Length;
        // 最多旋转 4 次
        for (int k = 0; k < 4; k++) {
            // 旋转操作
            for (int i = 0; i < n / 2; i++) {
                for (int j = 0; j < (n + 1) / 2; j++) {
                    int temp = mat[i][j];
                    mat[i][j] = mat[n-1-j][i];
                    mat[n-1-j][i] = mat[n-1-i][n-1-j];
                    mat[n-1-i][n-1-j] = mat[j][n-1-i];
                    mat[j][n-1-i] = temp;
                }
            }
            
            if (IsEqual(mat, target)) {
                return true;
            }
        }
        return false;
    }
    
    private bool IsEqual(int[][] mat, int[][] target) {
        int n = mat.Length;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (mat[i][j] != target[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
}