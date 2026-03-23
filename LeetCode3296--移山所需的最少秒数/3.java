class Solution {
    public long minNumberOfSeconds(int mountainHeight, int[] workerTimes) {
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        for (int t : workerTimes) {
            pq.offer(new long[]{t, t, t});
        }

        long ans = 0;
        while (mountainHeight-- > 0) {
            // 工作后总用时，当前工作（山高度降低 1）用时，workerTimes[i]
            long[] top = pq.poll();
            long total = top[0], cur = top[1], base = top[2];
            ans = total; // 最后一个出堆的 total 即为答案
            pq.offer(new long[]{total + cur + base, cur + base, base});
        }
        return ans;
    }
}