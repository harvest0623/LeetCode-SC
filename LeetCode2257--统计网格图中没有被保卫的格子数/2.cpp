class Solution {
    // 左右上下
    static constexpr int DIRS[4][2] = {{0, -1}, {0, 1}, {-1, 0}, {1, 0}};

public:
    int countUnguarded(int m, int n, vector<vector<int>>& guards, vector<vector<int>>& walls) {
        vector guarded(m, vector<int8_t>(n));

        // 标记警卫格子、墙格子
        for (auto& g : guards) {
            guarded[g[0]][g[1]] = -1;
        }
        for (auto& w : walls) {
            guarded[w[0]][w[1]] = -1;
        }

        // 遍历警卫
        for (auto& g : guards) {
            // 遍历视线方向（左右上下）
            for (auto& [dx, dy] : DIRS) {
                // 视线所及之处，被保卫
                int x = g[0] + dx, y = g[1] + dy;
                while (0 <= x && x < m && 0 <= y && y < n && guarded[x][y] != -1) {
                    guarded[x][y] = 1; // 被保卫
                    x += dx;
                    y += dy;
                }
            }
        }

        // 统计没被保卫（值为 0）的格子数
        int ans = 0;
        for (auto& row : guarded) {
            ans += ranges::count(row, 0);
        }
        return ans;
    }
};