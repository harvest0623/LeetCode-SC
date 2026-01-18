// 方法一：贪心
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut mod1 = Vec::new();
        let mut mod2 = Vec::new();
        
        for &num in &nums {
            total += num;
            if num % 3 == 1 {
                mod1.push(num);
            } else if num % 3 == 2 {
                mod2.push(num);
            }
        }
        
        mod1.sort();
        mod2.sort();
        
        if total % 3 == 0 {
            total
        } else if total % 3 == 1 {
            let mut remove = i32::MAX;
            if mod1.len() >= 1 {
                remove = remove.min(mod1[0]);
            }
            if mod2.len() >= 2 {
                remove = remove.min(mod2[0] + mod2[1]);
            }
            
            if remove == i32::MAX {
                0
            } else {
                total - remove
            }
        } else { // total % 3 == 2
            let mut remove = i32::MAX;
            if mod2.len() >= 1 {
                remove = remove.min(mod2[0]);
            }
            if mod1.len() >= 2 {
                remove = remove.min(mod1[0] + mod1[1]);
            }
            
            if remove == i32::MAX {
                0
            } else {
                total - remove
            }
        }
    }
}

// 方法二：动态规划
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = [0, i32::MIN, i32::MIN];      
        for num in nums {
            let mut new_dp = dp;  
            for i in 0..3 {
                if dp[i] != i32::MIN {
                    let new_sum = dp[i] + num;
                    let new_remainder = (new_sum % 3) as usize;
                    new_dp[new_remainder] = new_dp[new_remainder].max(new_sum);
                }
            }           
            dp = new_dp;
        }       
        if dp[0] > 0 { dp[0] } else { 0 }
    }
}