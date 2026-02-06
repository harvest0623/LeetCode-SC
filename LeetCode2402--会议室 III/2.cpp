class Solution {
public:
    int mostBooked(int n, vector<vector<int>>& meetings) {
        ranges::sort(meetings, {}, [](auto& m) { return m[0]; });

        priority_queue<int, vector<int>, greater<>> idle; // 会议室编号
        for (int i = 0; i < n; i++) {
            idle.push(i);
        }
        priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> using_; // (结束时间，会议室编号)
        vector<int> cnt(n); // 会议室的开会次数

        for (auto& m : meetings) {
            long long start = m[0], end = m[1];

            // 在 start 时刻空出来的会议室
            while (!using_.empty() && using_.top().first <= start) {
                idle.push(using_.top().second);
                using_.pop();
            }

            int i;
            if (!idle.empty()) { // 有空闲的会议室
                i = idle.top();
                idle.pop();
            } else { // 没有空闲的会议室
                // 弹出一个最早结束的会议室（若有多个同时结束，弹出编号最小的会议室）
                auto [e, room] = using_.top();
                i = room;
                using_.pop(); 
                end += e - start; // 更新当前会议的结束时间
            }

            using_.emplace(end, i); // 使用一个会议室
            cnt[i]++;
        }

        return ranges::max_element(cnt) - cnt.begin();
    }
};