class Solution {
    public int numSub(String s) {
        final int MOD = 1e9 + 7;
        long ans = 0;
        int last0 = -1;
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            if (ch == '0') {
                last0 = i; 
            } else {
                ans += i - last0; 
            }
        }
        return (int) (ans % MOD);
    }
}