impl Solution {
    fn pow(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let mut res = Self::pow(x, n / 2);
        res *= res;
        if n % 2 != 0 {
            res *= x;
        }
        res
    }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n < 0 {
            Self::pow(1.0 / x, -(n as i64))
        } else {
            Self::pow(x, n as i64)
        }
    }
}