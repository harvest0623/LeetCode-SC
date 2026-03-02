class Solution {
public:
    int numSteps(string s) {
        int ans = s.size() - 1;
        int i = s.find_last_of('1');
        if (i > 0) {
            ans += count(s.begin() + 1, s.begin() + i, '0') + 2;
        }
        return ans;
    }
};