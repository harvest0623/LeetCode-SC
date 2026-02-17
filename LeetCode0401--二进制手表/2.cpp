class Solution {
public:
    vector<string> readBinaryWatch(int turnedOn) {
        vector<string> ans;
        char s[6];
        for (uint8_t h = 0; h < 12; h++) {
            for (uint8_t m = 0; m < 60; m++) {
                if (popcount(h) + popcount(m) == turnedOn) {
                    sprintf(s, "%d:%02d", h, m);
                    ans.emplace_back(s);
                }
            }
        }
        return ans;
    }
};