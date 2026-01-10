// 方法一：正序遍历
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() - 1 {
            i += bits[i] as usize + 1;
        }
        i == bits.len() - 1
    }
}

// 方法二：倒序遍历
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = bits.len() - 2;
        while i >= 0 && bits[i] == 1 {
            i -= 1;
        }
        (bits.len() - i) % 2 == 0
    }
}