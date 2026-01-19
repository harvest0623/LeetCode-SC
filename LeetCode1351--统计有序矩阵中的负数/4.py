# 方法一：遍历 (O(m × n))
def countNegatives(grid):
    cnt = 0
    for row in grid:
        for val in row:
            if val < 0:
                cnt += 1
    return cnt

# 简便写法
class Solution(object):
    def countNegatives(self, grid):
        return sum(1 for row in grid for val in row if val < 0)

# 方法二：有序性 (O(m + n))
def countNegatives(grid):
    m, n = len(grid), len(grid[0])
    r, c = 0, n - 1
    cnt = 0
    while r < m and c >= 0:
        if grid[r][c] < 0:
            cnt += m - r
            c -= 1
        else:
            r += 1
    return cnt