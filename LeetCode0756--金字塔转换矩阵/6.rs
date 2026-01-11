use std::collections::HashSet;
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut t = [[0u8; 7]; 7];       
        for a in allowed {
            let chars: Vec<char> = a.chars().collect();
            let left = (chars[0] as u8 - b'A') as usize;
            let right = (chars[1] as u8 - b'A') as usize;
            let top = (chars[2] as u8 - b'A') as usize;
            t[left][right] |= 1 << top;
        }  
        let n = bottom.len();
        let mut a = vec![vec![0u8; n]; n];    
        for (i, c) in bottom.chars().enumerate() {
            a[n-1][i] = (c as u8 - b'A') as u8;
        }  
        let mut seen = HashSet::new();
        fn solve(
            t: &[[u8; 7]], 
            a: &mut Vec<Vec<u8>>, 
            seen: &mut HashSet<u64>, 
            r: u64, 
            n: usize, 
            i: usize
        ) -> bool {
            if n == 1 && i == 1 {
                return true;
            } else if i == n {
                if seen.contains(&r) {
                    return false;
                }
                seen.insert(r);
                return solve(t, a, seen, 0, n - 1, 0);
            } else {
                let w = t[a[n][i] as usize][a[n][i + 1] as usize];
                for b in 0..7 {
                    if (w >> b) & 1 != 0 {
                        a[n - 1][i] = b as u8;
                        if solve(t, a, seen, r * 8 + (b as u64 + 1), n, i + 1) {
                            return true;
                        }
                    }
                }
                false
            }
        }
        solve(&t, &mut a, &mut seen, 0, n - 1, 0)
    }
}