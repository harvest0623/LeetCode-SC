// 方法一：枚举两侧的字符
use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut res = 0;
        // 枚举两侧字符
        for ch in 'a'..='z' {
            // 使用迭代器找到第一个和最后一个出现位置
            let mut chars = s.chars();
            let l = chars.position(|c| c == ch);
            let r = chars.rev().position(|c| c == ch).map(|pos| s.len() - 1 - pos);
            if let (Some(l), Some(r)) = (l, r) {
                if r > l + 1 {
                    // 收集中间字符
                    let unique_chars: HashSet<_> = s[l+1..r].chars().collect();
                    res += unique_chars.len() as i32;
                }
            }
        }
        res
    }
}

// 方法二：枚举中间的字符  
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let mut res = 0;
        // 前缀/后缀字符状态数组
        let mut pre = vec![0u32; n];
        let mut suf = vec![0u32; n];
        for (i, c) in s.chars().enumerate() {
            // 前缀 s[0..i-1] 包含的字符种类
            pre[i] = if i > 0 { pre[i-1] } else { 0 } | (1 << (c as u8 - b'a'));
        }
        for (i, c) in s.chars().rev().enumerate() {
            let i = n - 1 - i;
            // 后缀 s[i+1..n-1] 包含的字符种类
            suf[i] = if i != n - 1 { suf[i+1] } else { 0 } | (1 << (c as u8 - b'a'));
        }
        // 每种中间字符的回文子序列状态数组
        let mut ans = vec![0u32; 26];
        for (i, c) in s.chars().enumerate() {
            if i > 0 && i < n - 1 {
                ans[(c as u8 - b'a') as usize] |= pre[i-1] & suf[i+1];
            }
        }
        // 更新答案
        for &count in &ans {
            res += count.count_ones() as i32;
        } 
        res
    }
}