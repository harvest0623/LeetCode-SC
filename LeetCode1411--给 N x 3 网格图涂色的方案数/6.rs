impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mod_val: i64 = 1e9 + 7;
        let mut preA: i64 = 6;
        let mut preB: i64 = 6;
        for _ in 2..= n {
            let new_preA = (2 * preA + 2 * preB) % mod_val;
            let new_preB = (2 * preA + 3 * preB) % mod_val;
            preA = new_preA;
            preB = new_preB;
        }
        ((preA + preB) % mod_val) as i32
    }
}