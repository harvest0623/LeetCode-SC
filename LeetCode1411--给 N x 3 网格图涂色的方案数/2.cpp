class Solution {
private:
    static constexpr int mod = 1e9 + 7;
public:
    int numOfWays(int n) {
        int preA = 6, preB = 6;
        for (int i = 2; i <= n; ++i) {
            int new_fi0 = (2LL * preA + 2LL * preB) % mod;
            int new_fi1 = (2LL * preA + 3LL * preB) % mod;
            preA = new_fi0;
            preB = new_fi1;
        }
        return (preA + preB) % mod;
    }
};