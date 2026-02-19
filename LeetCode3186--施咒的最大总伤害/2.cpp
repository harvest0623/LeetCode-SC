class Solution {
public:
    long long maximumTotalDamage(vector<int>& power) {
        unordered_map<int, int> cnt;
        for (int x : power) {
            cnt[x]++;
        }
        vector<pair<int, int>> a(cnt.begin(), cnt.end());
        ranges::sort(a);
        int n = a.size();
        vector<long long> f(n + 1);
        for (int i = 0, j = 0; i < n; i++) {
            auto& [x, c] = a[i];
            while (a[j].first < x - 2) {
                j++;
            }
            f[i + 1] = max(f[i], f[j] + (long long) x * c);
        }
        return f[n];
    }
};