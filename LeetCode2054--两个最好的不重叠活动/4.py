class Solution:
    def maxTwoEvents(self, events: List[List[int]]) -> int:
        # 按照结束时间排序
        events.sort(key=lambda e: e[1])  

        # 从栈底到栈顶，结束时间递增，价值递增
        st = [(0, 0)]  # 栈底哨兵 
        ans = 0
        for start_time, end_time, value in events:
            # 二分查找最后一个结束时间 < start_time 的活动
            i = bisect_left(st, (start_time,)) - 1
            ans = max(ans, st[i][1] + value)
            # 遇到比栈顶更大的价值，入栈
            if value > st[-1][1]:
                st.append((end_time, value))
        return ans