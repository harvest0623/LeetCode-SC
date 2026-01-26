// 方法一：暴力计算
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut week = 1;
        let mut day = 1;
        let mut res = 0;
        for _ in 0..n {
            res += week + day - 1;
            day += 1;
            if day == 8 {
                day = 1;
                week += 1;
            }
        }
        res
    }
}

// 方法二：等差数列求和
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        const D: i32 = 7;
        let w = n / D;
        let r = n % D;
        (w * D * (w + D) + r * (w * 2 + r + 1)) / 2
    }
}