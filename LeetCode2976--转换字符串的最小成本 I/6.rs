impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let mut g = [[0x3f3f3f3f; 26]; 26];
        for i in 0..26 {
            g[i][i] = 0;
        }
        for i in 0..original.len() {
            let x = original[i] as usize - 'a' as usize;
            let y = changed[i] as usize - 'a' as usize;
            g[x][y] = g[x][y].min(cost[i]);
        }
        for k in 0..26 {
            for i in 0..26 {
                if g[i][k] >= 0x3f3f3f3f {
                    continue;
                }
                for j in 0..26 {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }
        let mut ans = 0;
        let source_chars: Vec<char> = source.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();
        for i in 0..source.len() {
            let a = source_chars[i] as usize - 'a' as usize;
            let b = target_chars[i] as usize - 'a' as usize;
            if g[a][b] >= 0x3f3f3f3f {
                return -1;
            }
            ans += g[a][b] as i64;
        }
        ans
    }
}