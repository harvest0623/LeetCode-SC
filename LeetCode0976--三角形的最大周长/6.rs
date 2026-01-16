impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in (2..nums.len()).rev() {
            if nums[i - 2] + nums[i - 1] > nums[i] {
                return nums[i - 2] + nums[i - 1] + nums[i];
            }
        }
        0
    }
}