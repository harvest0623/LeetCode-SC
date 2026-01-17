impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![false; nums.len()];
        let mut x = 0;
        for (i, bit) in nums.into_iter().enumerate() {
            x = (x << 1 | bit) % 5;
            ans[i] = x == 0;
        }
        ans
    }
}