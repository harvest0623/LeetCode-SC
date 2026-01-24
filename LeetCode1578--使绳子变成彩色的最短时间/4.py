class Solution:
    def minCost(self, colors: str, neededTime: List[int]) -> int:
        i = 0
        length = len(colors)
        ret = 0
        while i < length:
            ch = colors[i]
            maxValue = 0
            total = 0
            while i < length and colors[i] == ch:
                maxValue = max(maxValue, neededTime[i])
                total += neededTime[i]
                i += 1 
            ret += total - maxValue
        return ret