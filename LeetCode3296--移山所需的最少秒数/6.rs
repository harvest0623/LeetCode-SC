const EPS: f64 = 1e-7;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mountain_height = mountain_height as i64;
        let max_worker_times = *worker_times.iter().max().unwrap_or(&0) as i64;
        
        let mut l: i64 = 1;
        let mut r: i64 = max_worker_times * mountain_height * (mountain_height + 1) / 2;
        let mut ans: i64 = 0;
        
        while l <= r {
            let mid = (l + r) / 2;
            let mut cnt: i64 = 0;
            
            for &t in &worker_times {
                let work = mid / t as i64;
                // 求最大的 k 满足 1+2+...+k <= work
                let k = ((-1.0 + (1.0 + (work as f64) * 8.0).sqrt()) / 2.0 + EPS) as i64;
                cnt += k;
            }
            
            if cnt >= mountain_height {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        
        ans
    }
}