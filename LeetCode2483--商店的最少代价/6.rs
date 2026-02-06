impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let n = customers.len();
        let chars: Vec<char> = customers.chars().collect();
        let mut suf = 0;
        let mut pre = 0;
        let mut min_cost = 0;
        let mut res = 0;
        for i in 0..=n {
            let cost = suf + pre;

            if min_cost > cost {
                min_cost = cost;
                res = i as i32;
            }

            if i < n {
                if chars[i] == 'N' {
                    pre += 1;
                } else {
                    suf -= 1;
                }
            }
        }
        res
    }
}