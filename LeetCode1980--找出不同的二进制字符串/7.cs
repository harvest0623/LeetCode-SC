public class Solution {
    public string FindDifferentBinaryString(string[] nums) {
        int n = nums.Length;
        // 预处理对应整数的哈希集合
        HashSet<int> vals = new HashSet<int>();
        foreach (string num in nums) {
            vals.Add(Convert.ToInt32(num, 2));
        }
        // 寻找第一个不在哈希集合中的整数
        int val = 0;
        while (vals.Contains(val)) {
            ++val;
        }
        // 将整数转化为二进制字符串返回
        string binary = Convert.ToString(val, 2);
        // 补齐前导0
        while (binary.Length < n) {
            binary = "0" + binary;
        }
        return binary;
    }
}