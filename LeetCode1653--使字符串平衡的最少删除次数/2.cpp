class Solution {
public:
    int minimumDeletions(string s) {
        int ans = 0;
        int cntA = 0, cnt = 0;
        for (char c : s) {
            cnt += c == 'a' ? -1 : 1;
            cntA += c == 'a';
            ans = min(ans, cnt);
        }
        return cntA + ans;
    }
};