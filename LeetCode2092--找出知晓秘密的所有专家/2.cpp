class Solution {
public:
    vector<int> findAllPeople(int n, vector<vector<int>>& meetings,
                              int firstPerson) {
        sort(meetings.begin(), meetings.end(),
             [](auto& a, auto& b) { return a[2] < b[2]; });
        vector<bool> know(n, false);
        know[0] = true;
        know[firstPerson] = true;
        int m = meetings.size();
        for (int i = 0; i < m;) {
            int t = meetings[i][2];
            unordered_map<int, int> parent;
            function<int(int)> find = [&](int x) {
                if (!parent.count(x))
                    parent[x] = x;
                if (parent[x] != x)
                    parent[x] = find(parent[x]);
                return parent[x];
            };
            auto unite = [&](int a, int b) {
                int pa = find(a), pb = find(b);
                if (pa != pb)
                    parent[pb] = pa;
            };
            int j = i;
            while (j < m && meetings[j][2] == t) {
                unite(meetings[j][0], meetings[j][1]);
                j++;
            }
            unordered_map<int, bool> hasSecret;
            for (auto& [x, p] : parent) {
                int root = find(x);
                if (know[x])
                    hasSecret[root] = true;
            }
            for (auto& [x, p] : parent) {
                if (hasSecret[find(x)]) {
                    know[x] = true;
                }
            }
            i = j;
        }
        vector<int> ans;
        for (int i = 0; i < n; i++) {
            if (know[i])
                ans.push_back(i);
        }
        return ans;
    }
};