impl Solution {
    pub fn count_collisions(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        // 前缀向左的车不会发生碰撞
        let mut l = 0;
        while l < n && s[l] == b'L' {
            l += 1;
        }
        // 后缀向右的车不会发生碰撞
        let mut r = n;
        while r > l && s[r - 1] == b'R' {
            r -= 1;
        }
        // 剩下非静止的车计入碰撞次数
        s[l..r].iter().filter(|&&c| c != b'S').count() as _
    }
}