// 方法一：排序
impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
        if nums.len() > 1 {
            let (first, rest) = nums.split_at_mut(1);
            rest.sort();
        }
        nums.iter().take(3).sum()
    }
}

// 方法二：维护最小值和次小值
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut n1 = i32::MAX;
        let mut n2 = i32::MAX;
        for i in 1..nums.len() {
            let x = nums[i];
            if x < n1 {
                n2 = n1;
                n1 = x;
            } else if x < n2 {
                n2 = x;
            }
        }
        nums[0] + n1 + n2
    }
}