use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        // 预处理对应整数的哈希集合
        let mut vals: HashSet<i32> = HashSet::new();
        for num in &nums {
            let val = i32::from_str_radix(num, 2).unwrap();
            vals.insert(val);
        }
        // 寻找第一个不在哈希集合中的整数
        let mut val = 0;
        while vals.contains(&val) {
            val += 1;
        }
        // 将整数转化为二进制字符串返回
        let mut binary = format!("{:b}", val);
        // 补齐前导0
        while binary.len() < n {
            binary = format!("0{}", binary);
        }
        binary
    }
}