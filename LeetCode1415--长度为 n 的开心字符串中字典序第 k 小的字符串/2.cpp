class Solution {
public:
    string getHappyString(int n, int k) {
        vector<char> chs = {'a', 'b', 'c'};
        string res;
        if (k > 3 * (1 << (n - 1))) {
            return res;
        }
        for (int i = 0; i < n; i++) {
            int count = 1 << (n - i - 1);
            for (char c : chs) {
                if (!res.empty() && res.back() == c) {
                    continue;
                }
                if (k <= count) {
                    res.push_back(c);
                    break;
                }
                k -= count;
            }
        }
        return res;
    }
};