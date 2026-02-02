// 方法一：排序
class Solution {
    public int findFinalValue(int[] nums, int original) {
        Arrays.sort(nums);
        for (int num : nums) {
            if (original == num) {
                original *= 2;
            }
        }
        return original;
    }
}

// 方法二：哈希表
class Solution {
    public int findFinalValue(int[] nums, int original) {
        Set<Integer> set = new HashSet<>();
        for (int num : nums) {
            set.add(num);
        }
        while (set.contains(original)) {
            original *= 2;
        }
        return original;
    }
}