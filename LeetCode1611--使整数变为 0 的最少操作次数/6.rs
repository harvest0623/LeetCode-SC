// 方法一：递归
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let x = (n as f64).log2().floor() as i32;
        (1 << (x + 1)) - 1 - Self::minimum_one_bit_operations(n - (1 << x))
    }
}

// 方法二：迭代
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut ans = 0;
        let mut sign = 1;
        for i in (0..30).rev() {
            if n & (1 << i) != 0 {
                ans += sign * ((1 << (i + 1)) - 1);
                sign = -sign;
            }
        }
        ans
    }
}

// 方法三：格雷码转二进制码
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut ans = 0;
        let mut n = n;
        while n != 0 {
            ans ^= n;
            n >>= 1;
        }
        ans
    }
}