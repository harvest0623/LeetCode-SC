class Solution {
public:
    int numSub(string s) {
        constexpr int MOD = 1e9 + 7;
        long long ans = 0;
        int last0 = -1;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == '0') {
                last0 = i; 
            } else {
                ans += i - last0; 
            }
        }
        return ans % MOD;
    }
};