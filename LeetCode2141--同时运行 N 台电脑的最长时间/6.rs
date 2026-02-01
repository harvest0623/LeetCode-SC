impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let sum: i64 = batteries.iter().map(|&x| x as i64).sum();
        let n = n as i64;
        let mut left: i64 = 0;
        let mut right: i64 = sum / n;
        let mut ans: i64 = 0;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mut total: i64 = 0;
            for &cap in &batteries {
                total += std::cmp::min(cap as i64, mid);
            }
            if total >= n * mid {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}