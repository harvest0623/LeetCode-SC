impl Solution {
    pub fn check_palindrome(s: &str, low: usize, high: usize) -> bool {
        let s = s.as_bytes();
        let (mut low, mut high) = (low, high);
        while low < high {
            if s[low] != s[high] {
                return false;
            }
            low += 1;
            high -= 1;
        }
        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let (mut low, mut high) = (0, s.len() - 1);
        while low < high {
            if s.as_bytes()[low] == s.as_bytes()[high] {
                low += 1;
                high -= 1;
            } else {
                return Solution::check_palindrome(&s, low, high - 1) || Solution::check_palindrome(&s, low + 1, high);
            }
        }
        true
    }
}