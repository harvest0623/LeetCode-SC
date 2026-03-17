class Solution {
    public String findDifferentBinaryString(String[] nums) {
        int n = nums.length;
        // 预处理对应整数的哈希集合
        Set<Integer> vals = new HashSet<>();
        for (String num : nums) {
            vals.add(Integer.parseInt(num, 2));
        }
        // 寻找第一个不在哈希集合中的整数
        int val = 0;
        while (vals.contains(val)) {
            ++val;
        }
        // 将整数转化为二进制字符串返回
        StringBuilder sb = new StringBuilder(Integer.toBinaryString(val));
        // 补齐前导0
        while (sb.length() < n) {
            sb.insert(0, "0");
        }
        return sb.toString();
    }
}