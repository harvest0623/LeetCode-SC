class Solution:
    def minNumberOfSeconds(self, mountainHeight: int, workerTimes: List[int]) -> int:
        h = [(t, t, t) for t in workerTimes]
        heapify(h)

        for _ in range(mountainHeight):
            # 工作后总用时，当前工作（山高度降低 1）用时，workerTimes[i]
            total, cur, base = h[0]
            heapreplace(h, (total + cur + base, cur + base, base))
        return total  # 最后一个出堆的 total 即为答案