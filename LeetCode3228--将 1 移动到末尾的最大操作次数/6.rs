impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut count_one = 0;
        let mut ans = 0;
        let mut i = 0;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        
        while i < n {
            if chars[i] == '0' {
                while i + 1 < n && chars[i + 1] == '0' {
                    i += 1;
                }
                ans += count_one;
            } else {
                count_one += 1;
            }
            i += 1;
        }
        ans
    }
}