class Solution {
    public int mostBooked(int n, int[][] meetings) {
        Arrays.sort(meetings, (a, b) -> a[0] - b[0]);

        PriorityQueue<Integer> idle = new PriorityQueue<>(); // 会议室编号
        for (int i = 0; i < n; i++) {
            idle.offer(i);
        }
        PriorityQueue<long[]> using = new PriorityQueue<>(
            (a, b) -> a[0] != b[0] ? Long.compare(a[0], b[0]) : Long.compare(a[1], b[1])
        ); // (结束时间，会议室编号)
        int[] cnt = new int[n]; // 会议室的开会次数

        for (int[] m : meetings) {
            long start = m[0];
            long end = m[1];

            // 在 start 时刻空出来的会议室
            while (!using.isEmpty() && using.peek()[0] <= start) {
                idle.offer((int) using.poll()[1]);
            }

            int i;
            if (!idle.isEmpty()) { // 有空闲的会议室
                i = idle.poll();
            } else { // 没有空闲的会议室
                long[] p = using.poll(); // 弹出一个最早结束的会议室（若有多个同时结束，弹出编号最小的会议室）
                end += p[0] - start; // 更新当前会议的结束时间
                i = (int) p[1];
            }

            using.offer(new long[]{end, i}); // 使用一个会议室
            cnt[i]++;
        }

        int ans = 0;
        for (int i = 1; i < n; i++) {
            if (cnt[i] > cnt[ans]) {
                ans = i;
            }
        }
        return ans;
    }
}