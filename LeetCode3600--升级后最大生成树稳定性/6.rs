const MAX_STABILITY: i32 = 200000;

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: i32,
    typ: i32,
}

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(p: &[usize]) -> Self {
        DSU {
            parent: p.to_vec(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    fn join(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px != py {
            self.parent[px] = py;
        }
    }
}

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut ans = -1;

        if edges.len() < n - 1 {
            return -1;
        }

        let mut sorted_edges: Vec<Edge> = edges
            .iter()
            .map(|edge| Edge {
                u: edge[0] as usize,
                v: edge[1] as usize,
                w: edge[2],
                typ: edge[3],
            })
            .collect();

        let mut must_edges = Vec::new();
        let mut optional_edges = Vec::new();

        for edge in sorted_edges {
            if edge.typ == 1 {
                must_edges.push(edge);
            } else {
                optional_edges.push(edge);
            }
        }

        if must_edges.len() > n - 1 {
            return -1;
        }

        optional_edges.sort_by(|a, b| b.w.cmp(&a.w));

        let mut selected_init = 0;
        let mut must_min_stability = MAX_STABILITY;
        let initial_parent: Vec<usize> = (0..n).collect();
        let mut dsu_init = DSU::new(&initial_parent);

        for &edge in &must_edges {
            if dsu_init.find(edge.u) == dsu_init.find(edge.v) || selected_init == n - 1 {
                return -1;
            }

            dsu_init.join(edge.u, edge.v);
            selected_init += 1;
            must_min_stability = must_min_stability.min(edge.w);
        }

        let mut l = 0;
        let mut r = must_min_stability;

        while l < r {
            let mid = l + ((r - l + 1) >> 1);
            let mut dsu = DSU::new(&dsu_init.parent);
            let mut selected = selected_init;
            let mut doubled_count = 0;

            for &edge in &optional_edges {
                if dsu.find(edge.u) == dsu.find(edge.v) {
                    continue;
                }

                if edge.w >= mid {
                    dsu.join(edge.u, edge.v);
                    selected += 1;
                } else if doubled_count < k && edge.w * 2 >= mid {
                    doubled_count += 1;
                    dsu.join(edge.u, edge.v);
                    selected += 1;
                } else {
                    break;
                }

                if selected == n - 1 {
                    break;
                }
            }

            if selected != n - 1 {
                r = mid - 1;
            } else {
                ans = mid;
                l = mid;
            }
        }

        ans
    }
}