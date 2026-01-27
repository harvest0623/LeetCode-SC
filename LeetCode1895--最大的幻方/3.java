class Solution {
    public int largestMagicSquare(int[][] grid) {
        int m = grid.length, n = grid[0].length;
        int[][] rowPrefix = new int[m + 1][n + 1];
        int[][] colPrefix = new int[m + 1][n + 1];   
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                rowPrefix[i + 1][j + 1] = rowPrefix[i + 1][j] + grid[i][j];
                colPrefix[i + 1][j + 1] = colPrefix[i][j + 1] + grid[i][j];
            }
        }    
        for (int k = Math.min(m, n); k > 1; k--) {
            for (int r = 0; r + k <= m; r++) {
                for (int c = 0; c + k <= n; c++) {
                    int target = rowPrefix[r + 1][c + k] - rowPrefix[r + 1][c];
                    boolean ok = true;                   
                    for (int i = r; i < r + k; i++) {
                        if (rowPrefix[i + 1][c + k] - rowPrefix[i + 1][c] != target) {
                            ok = false;
                            break;
                        }
                    }
                    if (!ok) continue;                   
                    for (int j = c; j < c + k; j++) {
                        if (colPrefix[r + k][j + 1] - colPrefix[r][j + 1] != target) {
                            ok = false;
                            break;
                        }
                    }
                    if (!ok) continue;  
                    int diag1 = 0, diag2 = 0;
                    for (int d = 0; d < k; d++) {
                        diag1 += grid[r + d][c + d];
                        diag2 += grid[r + d][c + k - 1 - d];
                    }
                    if (diag1 != target || diag2 != target) continue;
                    
                    return k;
                }
            }
        }
        return 1;
    }
}