class Solution:
    def sumFourDivisors(self, nums: List[int]) -> int:
        ans = 0
        for num in nums:
            cnt = sum = 0
            i = 1
            while i * i <= num:
                if num % i == 0:
                    cnt += 1
                    sum += i
                    if i * i != num:  
                        cnt += 1
                        sum += num // i
                i += 1
            if cnt == 4:
                ans += sum
        return ans