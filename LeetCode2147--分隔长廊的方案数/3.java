class Solution {
    private static final int mod = 1000000007;
    
    public int numberOfWays(String corridor) {
        int n = corridor.length();
        int prev = -1, cnt = 0, ans = 1;
        for (int i = 0; i < n; ++i) {
            if (corridor.charAt(i) == 'S') {
                ++cnt;
                if (cnt >= 3 && cnt % 2 == 1) {
                    ans = (int)((long)ans * (i - prev) % mod);
                }
                prev = i;
            }
        }
        if (cnt < 2 || cnt % 2 != 0) {
            ans = 0;
        }
        return ans;
    }
}