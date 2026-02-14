impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut cur = vec![poured as f64];
        for i in 1..=query_row as usize {
            let mut nxt = vec![0.0; i + 1];
            for (j, x) in cur.into_iter().enumerate() {
                if x > 1.0 { 
                    nxt[j] += (x - 1.0) / 2.0;
                    nxt[j + 1] += (x - 1.0) / 2.0;
                }
            }
            cur = nxt;
        }
        cur[query_glass as usize].min(1.0) 
    }
}