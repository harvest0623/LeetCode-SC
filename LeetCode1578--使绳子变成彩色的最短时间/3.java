class Solution {
    public int minCost(String colors, int[] neededTime) {
        int i = 0, len = colors.length();
        int ret = 0;
        while (i < len) {
            char ch = colors.charAt(i);
            int maxValue = 0;
            int sum = 0;
            while (i < len && colors.charAt(i) == ch) {
                maxValue = Math.max(maxValue, neededTime[i]);
                sum += neededTime[i];
                i++;
            }
            ret += sum - maxValue;
        }
        return ret;
    }
}