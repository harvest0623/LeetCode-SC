use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by_key(|v| v[0]);
        let n = n as usize;
        let mut avail_rooms = BinaryHeap::new();
        for i in 0..n {
            avail_rooms.push(Reverse(i));
        }
        let mut used_rooms = BinaryHeap::new();
        let mut used_count = vec![0; n];
        let mut cur_time = 0i64;
        for meeting in meetings {
            cur_time = cur_time.max(meeting[0] as i64);
            while let Some(&Reverse((end_time, room))) = used_rooms.peek() {
                if end_time <= cur_time {
                    avail_rooms.push(Reverse(room));
                    used_rooms.pop();
                } else {
                    break;
                }
            }
            if avail_rooms.is_empty() {
                cur_time = used_rooms.peek().unwrap().0.0;
                while let Some(&Reverse((end_time, room))) = used_rooms.peek() {
                    if end_time <= cur_time {
                        avail_rooms.push(Reverse(room));
                        used_rooms.pop();
                    } else {
                        break;
                    }
                }
            }
            let Reverse(room) = avail_rooms.pop().unwrap();
            used_count[room] += 1;
            used_rooms.push(Reverse((cur_time + (meeting[1] - meeting[0]) as i64, room)));
        }
        let mut ans = 0;
        for i in 1..n {
            if used_count[i] > used_count[ans] {
                ans = i;
            }
        }
        ans as i32
    }
}