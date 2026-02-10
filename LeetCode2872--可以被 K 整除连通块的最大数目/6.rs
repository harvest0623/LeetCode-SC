impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut result = 0;
        fn dfs(node: usize, parent: i32, graph: &Vec<Vec<usize>>, values: &Vec<i32>, k: i32, result: &mut i32) -> i64 {
            let mut sum = values[node] as i64;
            for &neighbor in &graph[node] {
                if neighbor as i32 != parent {
                    sum += dfs(neighbor, node as i32, graph, values, k, result);
                }
            }
            if sum % (k as i64) == 0 {
                *result += 1;
            }
            sum
        }
        dfs(0, -1, &graph, &values, k, &mut result);
        result
    }
}