impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let s0 = &strs[0];
        for (j, c) in s0.bytes().enumerate() { 
            for s in &strs { 
                if j == s.len() || s.as_bytes()[j] != c { 
                    return s0[..j].to_string();
                }
            }
        }
        s0.clone()
    }
}