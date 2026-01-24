impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut i = 0;
        let len = colors.len();
        let mut ret = 0;
        let colors = colors.chars().collect::<Vec<char>>();
        while i < len {
            let ch = colors[i];
            let mut max_value = 0;
            let mut sum = 0;
            while i < len && colors[i] == ch {
                max_value = max_value.max(needed_time[i]);
                sum += needed_time[i];
                i += 1;
            }
            ret += sum - max_value;
        }
        ret
    }
}