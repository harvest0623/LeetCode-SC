use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut a = numerator as i64;
        let mut b = denominator as i64;
        let sign = if a * b < 0 { "-" } else { "" };
        a = a.abs(); 
        b = b.abs();
        let mut q = a / b;
        let mut r = a % b;
        if r == 0 { 
            return format!("{}{}", sign, q);
        }
        let mut ans = format!("{}{}.", sign, q);
        let mut r_to_pos = HashMap::new();
        r_to_pos.insert(r, ans.len()); 
        while r != 0 {
            r *= 10;
            q = r / b;
            r %= b;
            ans.push((b'0' + q as u8) as char);
            if let Some(&pos) = r_to_pos.get(&r) { 
                return format!("{}({})", &ans[..pos], &ans[pos..]);
            }
            r_to_pos.insert(r, ans.len()); 
        }
        ans 
    }
}