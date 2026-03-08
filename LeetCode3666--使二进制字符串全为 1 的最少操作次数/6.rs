impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len() as i32;
        let z = s.iter().filter(|&&c| c == b'0').count() as i32;
        if z == 0 {
            return 0;
        }
        if k == n {
            return if z == n { 1 } else { -1 };
        }
        let mut ans = i32::MAX;
        
        // 情况一：操作次数 m 是偶数
        if z % 2 == 0 {
            let m = std::cmp::max(
                (z + k - 1) / k,
                (z + n - k - 1) / (n - k)
            );
            ans = m + (m % 2); // 把 m 往上调整为偶数
        }
        
        // 情况二：操作次数 m 是奇数
        if z % 2 == k % 2 {
            let m = std::cmp::max(
                (z + k - 1) / k,
                (n - z + n - k - 1) / (n - k)
            );
            let odd_m = if m % 2 == 1 { m } else { m + 1 }; // 把 m 往上调整为奇数
            ans = ans.min(odd_m);
        }
        if ans < i32::MAX { ans } else { -1 }
    }
}