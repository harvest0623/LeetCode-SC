use std::cmp;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by_key(|e| e[1]);
        
        // 存储 (结束时间, 最大价值)
        let mut dp: Vec<(i32, i32)> = vec![(0, 0)];
        let mut ans = 0;
        for event in events {
            let start = event[0];
            let end = event[1];
            let val = event[2];
            
            // 二分查找：找到最后一个结束时间 < start_time 的事件
            let idx = dp.partition_point(|&(et, _)| et < start);
            if idx > 0 {
                ans = cmp::max(ans, dp[idx - 1].1 + val);
            }
        
            // 更新dp
            if val > dp.last().unwrap().1 {
                dp.push((end, val));
            }
        }
        ans
    }
}