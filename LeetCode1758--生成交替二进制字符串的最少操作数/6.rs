impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut diff = 0;
        for (i, ch) in s.bytes().enumerate() {
            // 如果 i 是偶数，把 ch 变成 0，否则变成 1
            if ch - b'0' != (i % 2) as u8 {
                diff += 1;
            }
        }
        diff.min(s.len() - diff) as _
    }
}