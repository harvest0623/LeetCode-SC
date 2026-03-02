impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut ans = s.len() as i32 - 1;
        if let Some(i) = s.rfind('1') {
            if i > 0 {
                let count = s[1..i].chars().filter(|&c| c == '0').count() as i32;
                ans += count + 2;
            }
        }
        ans
    }
}