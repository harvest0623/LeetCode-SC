impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let mut cnt = vec![0i64; n + 1];
        
        for i in 0..n {
            let left = (0).max(i as i32 - r) as usize;
            let right = (n as i32).min(i as i32 + r + 1) as usize;
            cnt[left] += stations[i] as i64;
            cnt[right] -= stations[i] as i64;
        }
        
        let min_val = *stations.iter().min().unwrap() as i64;
        let sum_total = stations.iter().map(|&x| x as i64).sum::<i64>();
        let mut hi = sum_total + k as i64;
        let mut lo = min_val;
        let mut res = 0;
        
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if Self::check(&cnt, mid, r, k as i64) {
                res = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        res
    }
    
    fn check(cnt: &[i64], val: i64, r: i32, k: i64) -> bool {
        let n = cnt.len() - 1;
        let mut diff = cnt.to_vec();
        let mut sum = 0i64;
        let mut remaining = k;
        
        for i in 0..n {
            sum += diff[i];
            if sum < val {
                let add = val - sum;
                if remaining < add {
                    return false;
                }
                remaining -= add;
                let end = (n as i32).min(i as i32 + 2 * r + 1) as usize;
                diff[end] -= add;
                sum += add;
            }
        }
        true
    }
}