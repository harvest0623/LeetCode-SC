class Solution:
    def maxMatrixSum(self, matrix: List[List[int]]) -> int:
        n = len(matrix)
        cnt = 0   # 负数元素的数量
        total = 0   # 所有元素的绝对值之和
        mn = float("INF")   # 方阵元素的最小绝对值
        for i in range(n):
            for j in range(n):
                mn = min(mn, abs(matrix[i][j]))
                if matrix[i][j] < 0:
                    cnt += 1
                total += abs(matrix[i][j])
        # 按照负数元素的数量的奇偶性讨论
        if cnt % 2 == 0:
            return total
        else:
            return total - 2 * mn