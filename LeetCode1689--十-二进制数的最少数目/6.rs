impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut res = 0;
        for c in n.chars() {
            res = res.max(c as i32 - '0' as i32);
        }
        res
    }
}