struct TrieNode {
    son: [Option<Box<TrieNode>>; 26],
    sid: i32,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            son: Default::default(),
            sid: -1,
        }
    }
}
impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<String>, changed: Vec<String>, cost: Vec<i32>) -> i64 {
        let mut root = Box::new(TrieNode::new());
        let mut sid = 0;        
        let mut get_id = |s: &str| -> i32 {
            let mut node = &mut root;
            for ch in s.chars() {
                let idx = (ch as u8 - b'a') as usize;
                if node.son[idx].is_none() {
                    node.son[idx] = Some(Box::new(TrieNode::new()));
                }
                node = node.son[idx].as_mut().unwrap();
            }
            if node.sid < 0 {
                node.sid = sid;
                sid += 1;
            }
            node.sid
        };        
        // 初始化距离矩阵
        let m = cost.len();
        let inf = i32::MAX / 2;
        let mut dis = vec![vec![inf; m * 2]; m * 2];       
        for i in 0..m * 2 {
            dis[i][i] = 0;
        }
        
        for i in 0..m {
            let x = get_id(&original[i]) as usize;
            let y = get_id(&changed[i]) as usize;
            dis[x][y] = dis[x][y].min(cost[i]);
        }       
        // Floyd 算法求任意两点最短路
        for k in 0..sid as usize {
            for i in 0..sid as usize {
                if dis[i][k] == inf {
                    continue;
                }
                for j in 0..sid as usize {
                    dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
                }
            }
        }       
        let n = source.len();
        let inf64 = i64::MAX / 2;
        let mut f = vec![inf64; n + 1];
        f[n] = 0;        
        for i in (0..n).rev() {
            // 不修改 source[i]
            if source.as_bytes()[i] == target.as_bytes()[i] {
                f[i] = f[i + 1];
            } else {
                f[i] = inf64;
            }           
            let mut p = &root;
            let mut q = &root;           
            for j in i..n {
                let p_idx = (source.as_bytes()[j] - b'a') as usize;
                let q_idx = (target.as_bytes()[j] - b'a') as usize;               
                if p.son[p_idx].is_none() || q.son[q_idx].is_none() {
                    break;
                }               
                p = p.son[p_idx].as_ref().unwrap();
                q = q.son[q_idx].as_ref().unwrap();
                
                if p.sid < 0 || q.sid < 0 {
                    continue;
                }                
                // 修改从 i 到 j 的这一段
                let d = dis[p.sid as usize][q.sid as usize];
                if d < inf {
                    f[i] = f[i].min(d as i64 + f[j + 1]);
                }
            }
        }       
        if f[0] < inf64 {
            f[0]
        } else {
            -1
        }
    }
}