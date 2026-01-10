class Solution {
public:
    int minimumDeleteSum(string s1, string s2) {
        int m = s2.size();
        int total = reduce(s1.begin(), s1.end(), 0) + reduce(s2.begin(), s2.end(), 0);

        vector<int> f(m + 1);
        for (char x : s1) {
            int pre = 0; 
            for (int j = 0; j < m; j++) {
                int tmp = f[j + 1];
                if (x == s2[j]) {
                    f[j + 1] = pre + x;
                } else {
                    f[j + 1] = max(f[j + 1], f[j]);
                }
                pre = tmp;
            }
        }
        return total - f[m] * 2;
    }
};