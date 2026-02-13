# 方法一：排序
class Solution:
    def minimumCost(self, nums: List[int]) -> int:
        nums[1:] = sorted(nums[1:])
        return sum(nums[:3])

# 方法二：维护最小值和次小值
class Solution:
    def minimumCost(self, nums: List[int]) -> int:
        return nums[0] + sum(nsmallest(2, nums[1:]))