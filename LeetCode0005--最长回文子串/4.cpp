class Solution {
public:
    string longestPalindrome(string s) {
        int n = s.size();
        if (n < 2)
            return s;
        int maxLen = 1;
        int start = 0;
        vector<vector<int>> dp(n, vector<int>(n));
        for (int i = 0; i < n; ++i)
            dp[i][i] = true;
        for (int l = 2; l <= n; ++l) {
            for (int i = 0; i < n; ++i) {
                int j = l + i - 1;
                if (j >= n)
                    break;
                if (s[i] != s[j])
                    dp[i][j] = false;
                else {
                    if (j - i < 3)
                        dp[i][j] = true;
                    else
                        dp[i][j] = dp[i + 1][j - 1];
                }
                if (dp[i][j] && j - i + 1 > maxLen) {
                    maxLen = j - i + 1;
                    start = i;
                }
            }
        }
        return s.substr(start, maxLen);
    }
};