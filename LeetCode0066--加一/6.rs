impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for d in digits.iter_mut().rev() {
            if *d < 9 {
                *d += 1; 
                return digits;
            }
            *d = 0;
        }
        digits.push(0);
        digits[0] = 1;
        digits
    }
}