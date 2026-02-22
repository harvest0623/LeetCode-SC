impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut last = -1;
        let mut i = 0;
        while n > 0 {
            if n & 1 == 1 {
                if last != -1 {
                    ans = ans.max(i - last);
                }
                last = i;
            }
            n >>= 1;
            i += 1;
        }
        ans
    }
}