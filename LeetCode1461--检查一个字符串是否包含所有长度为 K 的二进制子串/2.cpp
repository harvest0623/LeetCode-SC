class Solution {
public:
    bool hasAllCodes(string s, int k) {
        if (s.size() < (1 << k) + k - 1) {
            return false;
        }
        unordered_set<string> exists;
        for (int i = 0; i + k <= s.size(); ++i) {
            exists.insert(move(s.substr(i, k)));
        }
        return exists.size() == (1 << k);
    }
};