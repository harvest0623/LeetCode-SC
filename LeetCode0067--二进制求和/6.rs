impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut ans = vec![];
        let mut i = a.len() as isize - 1;
        let mut j = b.len() as isize - 1;
        let mut carry = 0; 
        while i >= 0 || j >= 0 || carry > 0 {
            let x = if i >= 0 { a[i as usize] - b'0' } else { 0 };
            let y = if j >= 0 { b[j as usize] - b'0' } else { 0 };
            let sum = x + y + carry; 
            ans.push(sum % 2 + b'0');
            carry = sum / 2;
            i -= 1;
            j -= 1;
        }
        ans.reverse();
        unsafe { String::from_utf8_unchecked(ans) }
    }
}