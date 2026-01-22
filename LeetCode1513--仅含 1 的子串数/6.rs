impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MOD: i64 = 1e9 + 7;
        let mut ans = 0;
        let mut last0 = -1;
        for (i, ch) in s.bytes().enumerate() {
            if ch == b'0' {
                last0 = i as i32;  
            } else {
                ans += (i as i32 - last0) as i64;
            }
        }
        (ans % MOD) as _
    }
}