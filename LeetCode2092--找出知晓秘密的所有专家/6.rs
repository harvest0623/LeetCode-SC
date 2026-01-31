use std::collections::{VecDeque, HashMap, HashSet};

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let n = n as usize;
        let mut meetings = meetings;
        meetings.sort_by(|x, y| x[2].cmp(&y[2]));
        let mut secret = vec![false; n];
        secret[0] = true;
        secret[first_person as usize] = true;
        
        let mut vertices = HashSet::new();
        let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut i = 0;
        let m = meetings.len();
        while i < m {
            // meetings[i .. j] 为同一时间
            let mut j = i;
            while j + 1 < m && meetings[j + 1][2] == meetings[i][2] {
                j += 1;
            }
            
            vertices.clear();
            edges.clear();
            for k in i..=j {
                let x = meetings[k][0];
                let y = meetings[k][1];
                vertices.insert(x);
                vertices.insert(y);
                edges.entry(x).or_insert_with(Vec::new).push(y);
                edges.entry(y).or_insert_with(Vec::new).push(x);
            }
            
            let mut queue = VecDeque::new();
            for &u in &vertices {
                if secret[u as usize] {
                    queue.push_back(u);
                }
            }
            
            while let Some(u) = queue.pop_front() {
                if let Some(neighbors) = edges.get(&u) {
                    for &v in neighbors {
                        if !secret[v as usize] {
                            secret[v as usize] = true;
                            queue.push_back(v);
                        }
                    }
                }
            }
            
            i = j + 1;
        }
        
        let mut ans = Vec::new();
        for i in 0..n {
            if secret[i] {
                ans.push(i as i32);
            }
        }
        ans
    }
}