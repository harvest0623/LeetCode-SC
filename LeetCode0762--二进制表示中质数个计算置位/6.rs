impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right).filter(|i| [2, 3, 5, 7, 11, 13, 17, 19].contains(&i.count_ones())).count() as i32
    }
}