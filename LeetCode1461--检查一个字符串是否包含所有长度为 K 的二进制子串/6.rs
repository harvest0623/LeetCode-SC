use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let total = 1 << k;
        if s.len() < total + k - 1 {
            return false;
        }
        let mut exists = HashSet::new();
        for i in 0..=(s.len() - k) {
            exists.insert(&s[i..i + k]);
        }
        exists.len() == total
    }
}