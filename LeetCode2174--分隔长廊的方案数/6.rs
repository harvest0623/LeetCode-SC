impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MOD: i32 = 1000000007;
        let n = corridor.len();
        let mut prev = -1;
        let mut cnt = 0;
        let mut ans: i64 = 1;
        
        for (i, ch) in corridor.chars().enumerate() {
            if ch == 'S' {
                cnt += 1;
                if cnt >= 3 && cnt % 2 == 1 {
                    ans = (ans * (i as i64 - prev as i64)) % MOD as i64;
                }
                prev = i as i32;
            }
        }
        if cnt < 2 || cnt % 2 != 0 {
            return 0;
        }
        ans as i32
    }
}