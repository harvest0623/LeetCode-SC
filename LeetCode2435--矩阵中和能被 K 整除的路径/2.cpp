class Solution {
public:
    int numberOfPaths(vector<vector<int>>& grid, int k) {
        constexpr int MOD = 1'000'000'007;
        int m = grid.size(), n = grid[0].size();
        vector memo(m, vector(n, vector<int>(k, -1))); // -1 表示没有计算过

        auto dfs = [&](this auto&& dfs, int i, int j, int s) -> int {
            if (i < 0 || j < 0) { // 出界
                return 0;
            }
            int pre_s = ((s - grid[i][j]) % k + k) % k; // 保证模 k 结果非负
            if (i == 0 && j == 0) { // 起点
                return pre_s == 0; // pre_s == 0 说明 s == grid[i][j] % k
            }
            int& res = memo[i][j][s]; // 注意这里是引用
            if (res != -1) { // 之前计算过
                return res;
            }
            return res = (dfs(i - 1, j, pre_s) + dfs(i, j - 1, pre_s)) % MOD;
        };

        return dfs(m - 1, n - 1, 0);
    }
};