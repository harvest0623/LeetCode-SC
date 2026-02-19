impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut pre = 0;
        let mut cnt = 1;
        let mut ans = 0;
        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                cnt += 1;
            } else {
                ans += std::cmp::min(pre, cnt);
                pre = cnt;
                cnt = 1;
            }
        }
        ans += std::cmp::min(pre, cnt);
        ans
    }
}