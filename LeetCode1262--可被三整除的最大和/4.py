# 方法一：贪心
class Solution:
    def maxSumDivThree(self, nums: List[int]) -> int:
        total = sum(nums)
        mod1, mod2 = [], []
        
        for num in nums:
            if num % 3 == 1:
                mod1.append(num)
            elif num % 3 == 2:
                mod2.append(num)
        
        mod1.sort()
        mod2.sort()
        
        if total % 3 == 0:
            return total
        elif total % 3 == 1:
            remove = float('inf')
            if len(mod1) >= 1:
                remove = min(remove, mod1[0])
            if len(mod2) >= 2:
                remove = min(remove, mod2[0] + mod2[1])
            return total - (0 if remove == float('inf') else remove)
        else:  # total % 3 == 2
            remove = float('inf')
            if len(mod2) >= 1:
                remove = min(remove, mod2[0])
            if len(mod1) >= 2:
                remove = min(remove, mod1[0] + mod1[1])
            return total - (0 if remove == float('inf') else remove)

# 方法二：动态规划
class Solution:
    def maxSumDivThree(self, nums: List[int]) -> int:
        dp = [0, float('-inf'), float('-inf')]       
        for num in nums:
            new_dp = dp[:] 
            for i in range(3):
                if dp[i] != float('-inf'):
                    new_sum = dp[i] + num
                    new_remainder = new_sum % 3
                    new_dp[new_remainder] = max(new_dp[new_remainder], new_sum)           
            dp = new_dp        
        return dp[0] if dp[0] > 0 else 0