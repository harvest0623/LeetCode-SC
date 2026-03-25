impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let chs = ['a', 'b', 'c'];
        let mut res = String::new();
        if k > 3 * (1 << (n - 1)) {
            return res;
        }
        for i in 0..n {
            if res.len() != i as usize {
                break;
            }
            let count = 1 << (n - i - 1);
            for &c in &chs {
                if !res.is_empty() && res.as_bytes()[res.len() - 1] as char == c {
                    continue;
                }
                if k <= count {
                    res.push(c);
                    break;
                }
                k -= count;
            }
        }
        res
    }
}