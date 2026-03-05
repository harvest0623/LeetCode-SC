class Solution {
    public int concatenatedBinary(int n) {
        final int MOD = 1_000_000_007;
        long ans = 0;
        for (int i = 1; i <= n; i++) {
            int w = 32 - Integer.numberOfLeadingZeros(i);
            ans = (ans << w | i) % MOD;
        }
        return (int) ans;
    }
}