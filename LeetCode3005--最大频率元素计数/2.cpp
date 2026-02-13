class Solution {
public:
    int maxFrequencyElements(vector<int>& nums) {
        unordered_map<int, int> cnt;
        int ans = 0, max_cnt = 0;
        for (int x : nums) {
            int c = ++cnt[x];
            if (c > max_cnt) {
                ans = max_cnt = c;
            } else if (c == max_cnt) {
                ans += c;
            }
        }
        return ans;
    }
};