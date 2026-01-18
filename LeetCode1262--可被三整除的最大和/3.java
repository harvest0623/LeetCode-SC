// 方法一：贪心
class Solution {
    public int maxSumDivThree(int[] nums) {
        int total = 0;
        List<Integer> mod1 = new ArrayList<>();
        List<Integer> mod2 = new ArrayList<>();
        
        for (int num : nums) {
            total += num;
            if (num % 3 == 1) mod1.add(num);
            else if (num % 3 == 2) mod2.add(num);
        }
        
        Collections.sort(mod1);
        Collections.sort(mod2);
        
        if (total % 3 == 0) {
            return total;
        } else if (total % 3 == 1) {
            int remove = Integer.MAX_VALUE;
            if (mod1.size() >= 1) remove = Math.min(remove, mod1.get(0));
            if (mod2.size() >= 2) remove = Math.min(remove, mod2.get(0) + mod2.get(1));
            
            return total - (remove == Integer.MAX_VALUE ? 0 : remove);
        } else { // total % 3 == 2
            int remove = Integer.MAX_VALUE;
            if (mod2.size() >= 1) remove = Math.min(remove, mod2.get(0));
            if (mod1.size() >= 2) remove = Math.min(remove, mod1.get(0) + mod1.get(1));
            
            return total - (remove == Integer.MAX_VALUE ? 0 : remove);
        }
    }
}

// 方法二：动态规划
class Solution {
    public int maxSumDivThree(int[] nums) {
        int[] dp = {0, Integer.MIN_VALUE, Integer.MIN_VALUE};
        for (int num : nums) {
            int[] newDp = dp.clone();
            for (int i = 0; i < 3; i++) {
                if (dp[i] != Integer.MIN_VALUE) {
                    int newSum = dp[i] + num;
                    int newRemainder = newSum % 3;
                    newDp[newRemainder] = Math.max(newDp[newRemainder], newSum);
                }
            }           
            dp = newDp;
        }       
        return dp[0] > 0 ? dp[0] : 0;
    }
}