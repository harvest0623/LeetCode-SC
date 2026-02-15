// 方法一：逐位颠倒
impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut x = n;
        let mut rev = 0;
        
        for i in 0..32 {
            if x == 0 {
                break;
            }
            rev |= (x & 1) << (31 - i);
            x >>= 1;
        }
        
        rev
    }
}

// 方法二：位运算分治
impl Solution {
    pub fn reverse_bits(x: i32) -> i32 {
        const M1: u32 = 0x55555555; // 01010101010101010101010101010101
        const M2: u32 = 0x33333333; // 00110011001100110011001100110011
        const M4: u32 = 0x0f0f0f0f; // 00001111000011110000111100001111
        const M8: u32 = 0x00ff00ff; // 00000000111111110000000011111111

        let mut n = x as u32;
        
        n = (n >> 1 & M1) | (n & M1) << 1;
        n = (n >> 2 & M2) | (n & M2) << 2;
        n = (n >> 4 & M4) | (n & M4) << 4;
        n = (n >> 8 & M8) | (n & M8) << 8;
        (n >> 16 | n << 16) as i32
    }
}