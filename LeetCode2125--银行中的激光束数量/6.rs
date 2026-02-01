impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut last = 0;
        let mut ans = 0;
        for line in bank {
            let cnt = line.chars().filter(|&c| c == '1').count() as i32;
            if cnt != 0 {
                ans += last * cnt;
                last = cnt;
            }
        }
        ans
    }
}