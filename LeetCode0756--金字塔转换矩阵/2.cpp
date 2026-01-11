class Solution {
public:
    bool pyramidTransition(string bottom, vector<string>& allowed) {
        string groups[6][6];
        for (auto& s : allowed) {
            groups[s[0] - 'A'][s[1] - 'A'] += s[2];
        }
        int n = bottom.size();
        vector<string> pyramid(n);
        for (int i = 0; i < n - 1; i++) {
            pyramid[i].resize(i + 1);
        }
        pyramid[n - 1] = move(bottom);
        unordered_set<string> vis; // 访问标记
        auto dfs = [&](this auto&& dfs, int i, int j) -> bool {
            if (i < 0) {
                return true;
            }
            if (j == i + 1) {
                if (!vis.insert(pyramid[i]).second) { 
                    return false;
                }
                return dfs(i - 1, 0);
            }
            for (char top : groups[pyramid[i + 1][j] - 'A'][pyramid[i + 1][j + 1] - 'A']) {
                pyramid[i][j] = top;
                if (dfs(i, j + 1)) {
                    return true;
                }
            }
            return false;
        };
        return dfs(n - 2, 0);
    }
};