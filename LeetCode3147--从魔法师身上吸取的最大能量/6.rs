impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let k_usize = k as usize;
        let mut ans = i32::MIN;
        for i in (n - k_usize)..n {
            let mut sum = 0;
            let mut j = i;
            while j < n {
                sum += energy[j];
                if sum > ans {
                    ans = sum;
                }
                if j < k_usize {
                    break;
                }
                j -= k_usize;
            }
        }
        ans
    }
}