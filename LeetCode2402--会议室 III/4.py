class Solution:
    def mostBooked(self, n: int, meetings: List[List[int]]) -> int:
        meetings.sort(key=lambda m: m[0])

        idle = list(range(n))  # 会议室编号
        using = []  # (结束时间，会议室编号)
        cnt = [0] * n  # 会议室的开会次数

        for start, end in meetings:
            # 在 start 时刻空出来的会议室
            while using and using[0][0] <= start:
                heappush(idle, heappop(using)[1])

            if idle:  # 有空闲的会议室
                i = heappop(idle)
            else:  # 没有空闲的会议室
                e, i = heappop(using)  # 弹出一个最早结束的会议室（若有多个同时结束，弹出编号最小的会议室）
                end += e - start  # 更新当前会议的结束时间

            heappush(using, (end, i))  # 使用一个会议室
            cnt[i] += 1

        return cnt.index(max(cnt))