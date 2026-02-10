class Solution {
public:
    int maxKDivisibleComponents(int n, vector<vector<int>>& edges, vector<int>& values, int k) {
        vector<vector<int>> graph(n);
        for (auto& edge : edges) {
            int u = edge[0], v = edge[1];
            graph[u].push_back(v);
            graph[v].push_back(u);
        }

        int result = 0;
        function<long long(int, int)> dfs = [&](int node, int parent) -> long long {
            long long sum = values[node];
            for (int neighbor : graph[node]) {
                if (neighbor != parent) {
                    sum += dfs(neighbor, node);
                }
            }
            if (sum % k == 0) {
                ++result;
            }
            return sum;
        };
        dfs(0, -1);
        return result;
    }
};