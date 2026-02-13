// 方法一：排序
class Solution {
    public int minimumCost(int[] nums) {
        Arrays.sort(nums, 1, nums.length);
        return nums[0] + nums[1] + nums[2];
    }
}

// 方法二：维护最小值和次小值
class Solution {
    public int minimumCost(int[] nums) {
        int n1 = Integer.MAX_VALUE;
        int n2 = Integer.MAX_VALUE;
        for (int i = 1; i < nums.length; i++) {
            int x = nums[i];
            if (x < n1) {
                n2 = n1;
                n1 = x;
            } else if (x < n2) {
                n2 = x;
            }
        }
        return nums[0] + n1 + n2;
    }
}