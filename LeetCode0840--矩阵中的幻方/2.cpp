class Solution {
public:
    int numMagicSquaresInside(vector<vector<int>>& grid) {
        int rows = grid.size();
        int cols = grid[0].size();
        if (rows < 3 || cols < 3) return 0;
        int count = 0;
        for (int r = 0; r <= rows - 3; ++r) {
            for (int c = 0; c <= cols - 3; ++c) {
                if (isMagicSquare(grid, r, c)) {
                    count++;
                }
            }
        }
        return count;
    }
    
private:
    bool isMagicSquare(vector<vector<int>>& grid, int r, int c) {
        // 中心必须是 5
        if (grid[r + 1][c + 1] != 5) return false;
        
        // 检查是否为 1~9 不重复
        bool seen[10] = {false};
        for (int i = r; i < r + 3; ++i) {
            for (int j = c; j < c + 3; ++j) {
                int num = grid[i][j];
                if (num < 1 || num > 9 || seen[num]) return false;
                seen[num] = true;
            }
        }
        
        // 行和
        if (grid[r][c] + grid[r][c+1] + grid[r][c+2] != 15) return false;
        if (grid[r+1][c] + grid[r+1][c+1] + grid[r+1][c+2] != 15) return false;
        if (grid[r+2][c] + grid[r+2][c+1] + grid[r+2][c+2] != 15) return false;
        
        // 列和
        if (grid[r][c] + grid[r+1][c] + grid[r+2][c] != 15) return false;
        if (grid[r][c+1] + grid[r+1][c+1] + grid[r+2][c+1] != 15) return false;
        if (grid[r][c+2] + grid[r+1][c+2] + grid[r+2][c+2] != 15) return false;
        
        // 对角线
        if (grid[r][c] + grid[r+1][c+1] + grid[r+2][c+2] != 15) return false;
        if (grid[r][c+2] + grid[r+1][c+1] + grid[r+2][c] != 15) return false;
        
        return true;
    }
};