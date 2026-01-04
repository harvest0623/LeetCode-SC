class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        stack_size = 0
        for x in nums:
            if x != val:
                nums[stack_size] = x  
                stack_size += 1
        return stack_size