class Solution {
public:
    int concatenatedBinary(int n) {
        constexpr int MOD = 1'000'000'007;
        long long ans = 0;
        for (int i = 1; i <= n; i++) {
            int w = bit_width((uint32_t) i);
            ans = (ans << w | i) % MOD;
        }
        return ans;
    }
};