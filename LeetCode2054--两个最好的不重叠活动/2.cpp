class Solution {
public:
    int maxTwoEvents(vector<vector<int>>& events) {
        // 按 endTime 排序
        sort(events.begin(), events.end(),
             [](const vector<int>& a, const vector<int>& b) {
                 return a[1] < b[1];
             });
        int n = events.size();
        vector<int> maxValue(n);
        maxValue[0] = events[0][2];
        // 前缀最大 value
        for (int i = 1; i < n; i++) {
            maxValue[i] = max(maxValue[i - 1], events[i][2]);
        }
        int ans = 0;
        for (int i = 0; i < n; i++) {
            int curVal = events[i][2];
            int l = 0, r = i - 1, pos = -1;
            // 二分找最后一个 end < start
            while (l <= r) {
                int mid = (l + r) / 2;
                if (events[mid][1] < events[i][0]) {
                    pos = mid;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            if (pos != -1) {
                curVal += maxValue[pos];
            }
            ans = max(ans, curVal);
        }
        return ans;
    }
};