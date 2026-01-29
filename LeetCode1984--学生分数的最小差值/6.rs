impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if k == 1 {
            return 0;
        }
        nums.sort();
        let mut ans = i32::MAX;
        for i in 0..=(nums.len() - k) {
            ans = ans.min(nums[i + k - 1] - nums[i]);
        }
        ans
    }
}