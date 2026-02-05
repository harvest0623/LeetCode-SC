# 左右上下
DIRS = (0, -1), (0, 1), (-1, 0), (1, 0)

class Solution:
    def countUnguarded(self, m: int, n: int, guards: List[List[int]], walls: List[List[int]]) -> int:
        guarded = [[0] * n for _ in range(m)]

        # 标记警卫格子、墙格子
        for x, y in guards:
            guarded[x][y] = -1
        for x, y in walls:
            guarded[x][y] = -1

        # 遍历警卫
        for x0, y0 in guards:
            # 遍历视线方向（左右上下）
            for dx, dy in DIRS:
                # 视线所及之处，被保卫
                x, y = x0 + dx, y0 + dy
                while 0 <= x < m and 0 <= y < n and guarded[x][y] != -1:
                    guarded[x][y] = 1  # 被保卫
                    x += dx
                    y += dy

        # 统计没被保卫（值为 0）的格子数
        return sum(row.count(0) for row in guarded)