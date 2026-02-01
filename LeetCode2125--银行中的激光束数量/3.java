class Solution {
    public int numberOfBeams(String[] bank) {
        int last = 0, ans = 0;
        for (String line : bank) {
            int cnt = (int) line.chars().filter(ch -> ch == '1').count();
            if (cnt != 0) {
                ans += last * cnt;
                last = cnt;
            }
        }
        return ans;
    }
}