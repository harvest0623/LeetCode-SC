use std::collections::HashSet;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        
        let get_edges = |fences: &[i32], border: i32| -> HashSet<i32> {
            let mut points = vec![1];
            points.extend_from_slice(fences);
            points.push(border);
            points.sort_unstable();
            
            (0..points.len())
                .flat_map(|i| {
                    let pi = points[i];
                    points[i+1..].iter().map(move |&b| b - pi)
                })
                .collect()
        };
        
        let h_edges = get_edges(&h_fences, m);
        let v_edges = get_edges(&v_fences, n);
        
        h_edges.intersection(&v_edges)
            .max()
            .map_or(-1, |&e| ((e as i64).pow(2) % MOD) as i32)
    }
}