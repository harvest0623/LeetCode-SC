impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness;
        happiness.sort_by(|a, b| b.cmp(a));
        let mut ans: i64 = 0;
        for i in 0..(k as usize) {
            let val = happiness[i] as i64 - i as i64;
            ans += val.max(0);
        }
        ans
    }
}