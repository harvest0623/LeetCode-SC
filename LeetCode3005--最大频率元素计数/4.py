class Solution:
    def maxFrequencyElements(self, nums: List[int]) -> int:
        cnt = defaultdict(int)
        ans = max_cnt = 0
        for x in nums:
            cnt[x] += 1
            c = cnt[x]
            if c > max_cnt:
                ans = max_cnt = c
            elif c == max_cnt:
                ans += c
        return ans