class Solution:
    def minimumAbsDifference(self, arr: List[int]) -> List[List[int]]:
        arr.sort()
        min_diff = float('inf')
        # 找出最小差值
        for i in range(1, len(arr)):
            min_diff = min(min_diff, arr[i] - arr[i-1])
        # 收集结果
        res = []
        for i in range(1, len(arr)):
            if arr[i] - arr[i-1] == min_diff:
                res.append([arr[i-1], arr[i]])       
        return res