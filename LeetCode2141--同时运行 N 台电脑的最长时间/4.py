class Solution:
    def maxRunTime(self, n: int, batteries: List[int]) -> int:
        left, right, ans = 0, sum(batteries) // n, 0
        while left <= right:
            mid = (left + right) // 2
            total = 0
            for cap in batteries:
                total += min(cap, mid)
            if total >= n * mid:
                ans = mid
                left = mid + 1
            else:
                right = mid - 1
        return ans