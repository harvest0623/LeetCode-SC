// 方法一：哈希表
class Solution {
    public int[] getSneakyNumbers(int[] nums) {
        List<Integer> res = new ArrayList<>();
        Map<Integer, Integer> cnt = new HashMap<>();
        for (int x : nums) {
            cnt.put(x, cnt.getOrDefault(x, 0) + 1);
            if (cnt.get(x) == 2) res.add(x);
        }
        return res.stream().mapToInt(i -> i).toArray();
    }
};

// 方法二：位运算
class Solution {
    public int[] getSneakyNumbers(int[] nums) {
        int n = nums.length - 2;
        int y = 0;
        for (int x : nums) {
            y ^= x;
        }
        for (int i = 0; i < n; i++) {
            y ^= i;
        }
        int lowBit = y & -y;
        int x1 = 0, x2 = 0;
        for (int x : nums) {
            if ((x & lowBit) != 0) {
                x1 ^= x;
            } else {
                x2 ^= x;
            }
        }
        for (int i = 0; i < n; i++) {
            if ((i & lowBit) != 0) {
                x1 ^= i;
            } else {
                x2 ^= i;
            }
        }
        return new int[]{x1, x2};
    }
}