impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let w = n.ilog2() + 1;
        ((1 << w) - 1) ^ n
    }
}