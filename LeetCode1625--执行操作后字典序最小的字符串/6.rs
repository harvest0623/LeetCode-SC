// 方法一：BFS 搜索 + 转轮
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let b = b as usize;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(s.clone());
        visited.insert(s.clone());
        let mut ans = s.clone();
        
        while let Some(cur) = queue.pop_front() {
            if cur < ans {
                ans = cur.clone();
            }
            
            // 操作1
            let mut op1_chars: Vec<char> = cur.chars().collect();
            for i in (1..n).step_by(2) {
                let digit = op1_chars[i].to_digit(10).unwrap() as i32;
                let new_digit = (digit + a) % 10;
                op1_chars[i] = std::char::from_digit(new_digit as u32, 10).unwrap();
            }
            let op1: String = op1_chars.into_iter().collect();
            if visited.insert(op1.clone()) {
                queue.push_back(op1);
            }
            
            // 操作2
            let op2 = format!("{}{}", &cur[n - b..], &cur[..n - b]);
            if visited.insert(op2.clone()) {
                queue.push_back(op2);
            }
        }
        ans
    }
}

// 方法二：数学 + 法转轮
impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let g = Self::gcd(a, 10);
        let step = Self::gcd(n as i32, b) as usize;
        let mut ans = s.clone();
        let mut change = |t: &mut Vec<char>, start: usize| {
            let ch = t[start].to_digit(10).unwrap() as i32;
            let inc = (ch % g + 10 - ch) % 10;
            for j in (start..n).step_by(2) {
                let digit = (t[j].to_digit(10).unwrap() as i32 + inc) % 10;
                t[j] = std::char::from_digit(digit as u32, 10).unwrap();
            }
        };
        for i in (0..n).step_by(step) {
            // 轮转
            let rotated = format!("{}{}", &s[i..], &s[..i]);
            let mut t: Vec<char> = rotated.chars().collect();
            change(&mut t, 1);  // 调整奇数位
            if step & 1 == 1 {
                change(&mut t, 0);  // 调整偶数位
            }            
            let candidate: String = t.into_iter().collect();
            if candidate < ans {
                ans = candidate;
            }
        }       
        ans
    }
}