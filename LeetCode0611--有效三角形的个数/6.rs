// 方法一
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        for k in 2..nums.len() {
            let c = nums[k];
            let mut i = 0; 
            let mut j = k - 1; 
            while i < j {
                if nums[i] + nums[j] > c {
                    ans += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        ans as _
    }
}

// 方法二
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        for k in 2..nums.len() {
            let c = nums[k];
            let mut i = 0; 
            let mut j = k - 1; 
            while i < j {
                if nums[i] + nums[j] > c {
                    ans += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        ans as _
    }
}