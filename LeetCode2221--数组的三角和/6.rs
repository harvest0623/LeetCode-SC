impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut current = nums.clone();
        while current.len() > 1 {
            let mut new_nums = Vec::with_capacity(current.len() - 1);
            for i in 0..current.len() - 1 {
                new_nums.push((current[i] + current[i + 1]) % 10);
            }
            current = new_nums;
        }
        current[0]
    }
}