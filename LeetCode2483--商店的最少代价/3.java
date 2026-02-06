class Solution {
    public int bestClosingTime(String customers) {
        int n = customers.length();
        int suf = 0;
        int pre = 0;
        int minCost = 0;
        int res = 0;
        for (int i = 0; i <= n; i++) {
            if (minCost > suf + pre) {
                minCost = suf + pre;
                res = i;
            }
            if (i < n && customers.charAt(i) == 'N') {
                pre++;
            } else if (i < n) {
                suf--;
            }
        }
        return res;
    }
}