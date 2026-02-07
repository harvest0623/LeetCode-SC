impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut ans = 0;
        let mut cnt_a = 0;
        let mut cnt = 0;
        for ch in s.chars() {
            if ch == 'a' {
                cnt -= 1;
                cnt_a += 1;
            } else {
                cnt += 1;
            }
            ans = ans.min(cnt);
        }
        cnt_a + ans
    }
}