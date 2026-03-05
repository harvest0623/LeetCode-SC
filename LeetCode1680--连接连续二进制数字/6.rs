impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let m = 1e9 as i64 + 7;
        let mut f:i64 = 1;
        let mut res: i64 = 0;
        for mut i in (1..n+1).rev() {
            res += (i as i64 * f) % m;
            while i > 0 {
                f = (f * 2) % m;
                i /= 2;
            }
        }
        (res % m) as i32
    }
}