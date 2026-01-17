class Solution:
    def smallestRepunitDivByK(self, k: int) -> int:
        if k % 2 == 0 or k % 5 == 0: 
            return -1
        ans, resid = 1, 1  
        while resid % k != 0: 
            resid = (resid % k) * (10 % k) + 1 
            ans += 1  
        return ans  