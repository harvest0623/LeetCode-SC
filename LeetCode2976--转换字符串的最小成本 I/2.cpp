class Solution {
public:
    long long minimumCost(string source, string target, vector<char>& original, vector<char>& changed, vector<int>& cost) {
        int g[26][26];
        memset(g, 0x3f, sizeof(g));
        for (int i = 0; i < 26; i++) g[i][i] = 0;
        for (int i = 0; i < original.size(); i++) {
            int x = original[i] - 'a', y = changed[i] - 'a';
            g[x][y] = min(g[x][y], cost[i]);
        }
        for (int k = 0; k < 26; k++) {
            for (int i = 0; i < 26; i++) {
                if (g[i][k] >= 0x3f3f3f3f) continue;
                for (int j = 0; j < 26; j++) {
                    g[i][j] = min(g[i][j], g[i][k] + g[k][j]);
                }
            }
        }
        long long ans = 0;
        for (int i = 0; i < source.size(); i++) {
            int a = source[i] - 'a', b = target[i] - 'a';
            if (g[a][b] >= 0x3f3f3f3f) return -1;
            ans += g[a][b];
        }
        return ans;
    }
};