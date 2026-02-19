class Solution {
public:
    int countBinarySubstrings(string& s) {
        int n = s.size();
        int pre = 0, cnt = 1, ans = 0;
        int i = 0;
        while (i < n) {
            while (i < n && s[i + 1] == s[i]) {
                ++cnt;
                ++i;
            }
            ans += min(pre, cnt);
            pre = cnt;
            cnt = 1;
            ++i;
        }
        return ans;
    }
};