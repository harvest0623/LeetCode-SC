impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let sqrt_i32_max = (i32::MAX as f64).sqrt() as i32;
        let mut left = 0;
        let mut right = x.min(sqrt_i32_max) + 1;
        while left + 1 < right { 
            let m = (left + right) / 2;
            if m * m <= x {
                left = m;
            } else {
                right = m;
            }
        }
        left
    }
}