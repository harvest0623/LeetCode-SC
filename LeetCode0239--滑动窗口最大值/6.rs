use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ans = Vec::with_capacity(nums.len() - k + 1); 
        let mut q = VecDeque::new();

        for (i, &x) in nums.iter().enumerate() {
            while !q.is_empty() && nums[*q.back().unwrap()] <= x {
                q.pop_back(); 
            }
            q.push_back(i); 

            if q[0] + k <= i {
                q.pop_front();
            }

            if i + 1 >= k {
                ans.push(nums[q[0]]);
            }
        }
        ans
    }
}