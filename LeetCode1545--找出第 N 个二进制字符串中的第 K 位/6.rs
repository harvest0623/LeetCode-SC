impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }
        if k == 1 << (n - 1) {
            return '1';
        }
        if k < 1 << (n - 1) {
            return Self::find_kth_bit(n - 1, k);
        }
        (Self::find_kth_bit(n - 1, (1 << n) - k) as u8 ^ 1) as _
    }
}