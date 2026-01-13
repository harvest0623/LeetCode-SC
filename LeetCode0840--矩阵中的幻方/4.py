class Solution:
    def numMagicSquaresInside(self, grid):
        rows, cols = len(grid), len(grid[0])
        if rows < 3 or cols < 3:
            return 0
        
        def is_magic(r, c):
            if grid[r+1][c+1] != 5:
                return False
            
            seen = [False] * 10
            for i in range(r, r+3):
                for j in range(c, c+3):
                    num = grid[i][j]
                    if num < 1 or num > 9 or seen[num]:
                        return False
                    seen[num] = True
            
            # 行
            for i in range(3):
                if sum(grid[r+i][c+j] for j in range(3)) != 15:
                    return False
            # 列
            for j in range(3):
                if sum(grid[r+i][c+j] for i in range(3)) != 15:
                    return False
            # 对角线
            if grid[r][c] + grid[r+1][c+1] + grid[r+2][c+2] != 15:
                return False
            if grid[r][c+2] + grid[r+1][c+1] + grid[r+2][c] != 15:
                return False
            
            return True
        
        count = 0
        for r in range(rows - 2):
            for c in range(cols - 2):
                if is_magic(r, c):
                    count += 1
        return count