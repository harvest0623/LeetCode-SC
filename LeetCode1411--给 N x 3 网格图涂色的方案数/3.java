class Solution {
    static final int MOD = 1e9 + 7;
    public int numOfWays(int n) {
        long preA = 6, preB = 6;
        for (int i = 2; i <= n; ++i) {
            long newPreA = (2 * preA + 2 * preB) % MOD;
            long newPreB = (2 * preA + 3 * preB) % MOD;
            preA = newPreA;
            preB = newPreB;
        }   
        return (int) ((preA + preB) % MOD);
    }
}