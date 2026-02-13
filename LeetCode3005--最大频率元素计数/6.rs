use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        let mut max_cnt = 0;
        let mut ans = 0;
        for x in nums {
            let e = cnt.entry(x).or_insert(0);
            *e += 1;
            let c = *e;
            if c > max_cnt {
                max_cnt = c;
                ans = c;
            } else if c == max_cnt {
                ans += c;
            }
        }
        ans
    }
}