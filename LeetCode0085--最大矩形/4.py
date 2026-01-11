# 方法一: 使用柱状图的优化暴力方法
class Solution:
    def maximalRectangle(self, matrix: list[list[str]]) -> int:
        if not matrix:
            return 0
        m, n = len(matrix), len(matrix[0])
        left = [[0] * n for _ in range(m)]
        for i in range(m):
            for j in range(n):
                if matrix[i][j] == '1':
                    left[i][j] = (0 if j == 0 else left[i][j - 1]) + 1
        ret = 0
        for i in range(m):
            for j in range(n):
                if matrix[i][j] == '0':
                    continue
                width = left[i][j]
                area = width
                for k in range(i - 1, -1, -1):
                    width = min(width, left[k][j])
                    area = max(area, (i - k + 1) * width)
                ret = max(ret, area)
        return ret

# 方法二：单调栈
class Solution:
    def maximalRectangle(self, matrix: List[List[str]]) -> int:
        if not matrix:
            return 0
        m, n = len(matrix), len(matrix[0])
        left = [[0] * n for _ in range(m)]

        for i in range(m):
            for j in range(n):
                if matrix[i][j] == '1':
                    left[i][j] = (0 if j == 0 else left[i][j - 1]) + 1

        ret = 0
        for j in range(n):
            up = [0] * m
            down = [0] * m
            stk = []

            for i in range(m):
                while stk and left[stk[-1]][j] >= left[i][j]:
                    stk.pop()
                up[i] = stk[-1] if stk else -1
                stk.append(i)

            stk = []
            for i in range(m - 1, -1, -1):
                while stk and left[stk[-1]][j] >= left[i][j]:
                    stk.pop()
                down[i] = stk[-1] if stk else m
                stk.append(i)

            for i in range(m):
                height = down[i] - up[i] - 1
                ret = max(ret, height * left[i][j])

        return ret