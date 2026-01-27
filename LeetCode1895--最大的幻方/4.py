class Solution:
    def largestMagicSquare(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        row_prefix = [[0] * (n + 1) for _ in range(m + 1)]
        col_prefix = [[0] * (n + 1) for _ in range(m + 1)]
        for i in range(m):
            for j in range(n):
                row_prefix[i + 1][j + 1] = row_prefix[i + 1][j] + grid[i][j]
                col_prefix[i + 1][j + 1] = col_prefix[i][j + 1] + grid[i][j]
        
        def get_row_sum(r, c1, c2):
            return row_prefix[r + 1][c2 + 1] - row_prefix[r + 1][c1]
        
        def get_col_sum(c, r1, r2):
            return col_prefix[r2 + 1][c + 1] - col_prefix[r1][c + 1]
        
        for k in range(min(m, n), 1, -1):
            for r in range(m - k + 1):
                for c in range(n - k + 1):
                    target = get_row_sum(r, c, c + k - 1)
                    ok = True
                    
                    for i in range(r, r + k):
                        if get_row_sum(i, c, c + k - 1) != target:
                            ok = False
                            break
                    if not ok:
                        continue
                    
                    for j in range(c, c + k):
                        if get_col_sum(j, r, r + k - 1) != target:
                            ok = False
                            break
                    if not ok:
                        continue                    
                    diag1 = sum(grid[r + d][c + d] for d in range(k))
                    diag2 = sum(grid[r + d][c + k - 1 - d] for d in range(k))
                    if diag1 != target or diag2 != target:
                        continue                    
                    return k
        return 1