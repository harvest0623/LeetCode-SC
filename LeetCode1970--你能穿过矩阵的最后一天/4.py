class Solution:
    def latestDayToCross(self, row: int, col: int, cells: List[List[int]]) -> int:
        left, right, ans = 0, row * col, 0
        while left <= right:
            mid = (left + right) // 2
            
            grid = [[1] * col for _ in range(row)]
            for x, y in cells[:mid]:
                grid[x - 1][y - 1] = 0

            q = deque()
            for i in range(col):
                if grid[0][i]:
                    q.append((0, i))
                    grid[0][i] = 0
            
            found = False
            while q:
                x, y = q.popleft()
                for nx, ny in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
                    if 0 <= nx < row and 0 <= ny < col and grid[nx][ny]:
                        if nx == row - 1:
                            found = True
                            break
                        q.append((nx, ny))
                        grid[nx][ny] = 0
            
            if found:
                ans = mid
                left = mid + 1
            else:
                right = mid - 1     
        return ans