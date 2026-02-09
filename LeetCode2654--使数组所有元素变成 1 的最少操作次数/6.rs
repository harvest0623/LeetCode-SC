impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a.abs()
        }

        let n = nums.len() as i32;
        let mut num1 = 0;
        let mut g = 0;

        for &x in &nums {
            if x == 1 {
                num1 += 1;
            }
            g = gcd(g, x);
        }

        if num1 > 0 {
            return n - num1;
        }
        if g > 1 {
            return -1;
        }

        let mut min_len = n;
        for i in 0..nums.len() {
            let mut g = 0;
            for j in i..nums.len() {
                g = gcd(g, nums[j]);
                if g == 1 {
                    min_len = std::cmp::min(min_len, (j - i + 1) as i32);
                    break;
                }
            }
        }

        min_len + n - 2
    }
}