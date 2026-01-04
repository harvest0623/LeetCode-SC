impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut stack_size = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[stack_size] = nums[i]; 
                stack_size += 1;
            }
        }
        stack_size as _
    }
}