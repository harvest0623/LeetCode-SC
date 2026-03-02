class Solution {
    public int numSteps(String s) {
        int ans = s.length() - 1;
        int i = s.lastIndexOf('1');
        if (i > 0) {
            int count = 0;
            for (int j = 1; j < i; j++) {
                if (s.charAt(j) == '0') {
                    count++;
                }
            }
            ans += count + 2;
        }
        return ans;
    }
}