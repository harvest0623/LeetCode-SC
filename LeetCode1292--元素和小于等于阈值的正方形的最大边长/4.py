class Solution:
    def maxSideLength(self, mat: List[List[int]], threshold: int) -> int:
        m, n = len(mat), len(mat[0])
        # 二维前缀和
        prefix = [[0] * (n + 1) for _ in range(m + 1)]
        
        for i in range(1, m + 1):
            for j in range(1, n + 1):
                prefix[i][j] = (prefix[i-1][j] + prefix[i][j-1] - 
                               prefix[i-1][j-1] + mat[i-1][j-1])
        
        # 二分查找最大边长
        left, right = 0, min(m, n)
        ans = 0
        
        while left <= right:
            mid = (left + right) // 2
            found = False
            
            # 检查所有边长为mid的正方形
            for i in range(1, m - mid + 2):
                for j in range(1, n - mid + 2):
                    x2, y2 = i + mid - 1, j + mid - 1
                    total = (prefix[x2][y2] - prefix[i-1][y2] - 
                            prefix[x2][j-1] + prefix[i-1][j-1])
                    if total <= threshold:
                        found = True
                        break
                if found:
                    break            
            if found:
                ans = mid
                left = mid + 1
            else:
                right = mid - 1        
        return ans