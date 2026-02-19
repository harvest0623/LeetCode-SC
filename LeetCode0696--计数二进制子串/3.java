class Solution {
    public int countBinarySubstrings(String s) {
        int pre = 0, cnt = 1, ans = 0;
        for (int i = 0; i < s.length() - 1; i++) {
            if (s.charAt(i) == s.charAt(i + 1)) {
                cnt++;
            } else {
                ans += Math.min(pre, cnt);
                pre = cnt;
                cnt = 1;
            }
        }
        ans += Math.min(pre, cnt);
        return ans;
    }
}