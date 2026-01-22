// 方法一：模拟
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut bottle = num_bottles;
        let mut ans = num_bottles;
        while bottle >= num_exchange {
            bottle -= num_exchange;
            ans += 1;
            bottle += 1;
        }
        ans
    }
}

// 方法二：数学
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        if num_bottles < num_exchange {
            return num_bottles
        }
        (num_bottles - num_exchange) / (num_exchange - 1) + 1 + num_bottles
    }
}