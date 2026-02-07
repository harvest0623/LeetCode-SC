class Solution {
    public int minimumDeletions(String s) {
        int ans = 0;
        int cntA = 0, cnt = 0;
        for (char c : s.toCharArray()) {
            cnt += c == 'a' ? -1 : 1;
            cntA += c == 'a' ? 1 : 0; 
            ans = Math.min(ans, cnt);
        }
        return cntA + ans;
    }
}