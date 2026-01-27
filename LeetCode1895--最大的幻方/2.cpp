class Solution {
public:
    int largestMagicSquare(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<int>> rowPrefix(m + 1, vector<int>(n + 1, 0));
        vector<vector<int>> colPrefix(m + 1, vector<int>(n + 1, 0));       
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                rowPrefix[i + 1][j + 1] = rowPrefix[i + 1][j] + grid[i][j];
                colPrefix[i + 1][j + 1] = colPrefix[i][j + 1] + grid[i][j];
            }
        }       
        auto getRowSum = [&](int r, int c1, int c2) {
            return rowPrefix[r + 1][c2 + 1] - rowPrefix[r + 1][c1];
        };       
        auto getColSum = [&](int c, int r1, int r2) {
            return colPrefix[r2 + 1][c + 1] - colPrefix[r1][c + 1];
        };       
        for (int k = min(m, n); k > 1; k--) {
            for (int r = 0; r + k <= m; r++) {
                for (int c = 0; c + k <= n; c++) {
                    int target = getRowSum(r, c, c + k - 1);
                    bool ok = true;
                    for (int i = r; i < r + k; i++) {
                        if (getRowSum(i, c, c + k - 1) != target) {
                            ok = false;
                            break;
                        }
                    }
                    if (!ok) continue;
                    for (int j = c; j < c + k; j++) {
                        if (getColSum(j, r, r + k - 1) != target) {
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
};