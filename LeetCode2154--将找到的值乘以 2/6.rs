// 方法一：排序
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut original = original;
        for &num in nums.iter() {
            if original == num {
                original *= 2;
            }
        }
        original
    }
}

// 方法二：哈希表
use std::collections::HashSet;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let set: HashSet<_> = nums.into_iter().collect();
        let mut original = original;
        while set.contains(&original) {
            original *= 2;
        }
        original
    }
}