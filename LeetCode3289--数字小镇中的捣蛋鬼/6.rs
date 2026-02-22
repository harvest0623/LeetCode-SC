// 方法一：哈希表
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut count = std::collections::HashMap::new();
        for x in nums {
            let c = count.entry(x).or_insert(0);
            *c += 1;
            if *c == 2 {
                res.push(x);
            }
        }
        res
    }
}

// 方法二：位运算
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let mut y = 0;
        for &x in &nums {
            y ^= x;
        }
        for i in 0..n {
            y ^= i;
        }
        let low_bit = y & -y;
        let mut x1 = 0;
        let mut x2 = 0;
        for &x in &nums {
            if x & low_bit != 0 {
                x1 ^= x;
            } else {
                x2 ^= x;
            }
        }
        for i in 0..n {
            if i & low_bit != 0 {
                x1 ^= i;
            } else {
                x2 ^= i;
            }
        }
        vec![x1, x2]
    }
}