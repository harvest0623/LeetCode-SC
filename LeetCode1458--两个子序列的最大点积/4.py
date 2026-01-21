class Solution:
    def maxDotProduct(self, nums1: List[int], nums2: List[int]) -> int:
        m, n = len(nums1), len(nums2)
        f = [[0] * n for _ in range(m)]
        for i in range(m):
            for j in range(n):
                xij = nums1[i] * nums2[j]
                f[i][j] = xij
                if i > 0:
                    f[i][j] = max(f[i][j], f[i - 1][j])
                if j > 0:
                    f[i][j] = max(f[i][j], f[i][j - 1])
                if i > 0 and j > 0:
                    f[i][j] = max(f[i][j], f[i - 1][j - 1] + xij)
        return f[m - 1][n - 1]