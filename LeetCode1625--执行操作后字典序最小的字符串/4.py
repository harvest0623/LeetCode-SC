# 方法一：BFS 搜索 + 转轮
class Solution:
    def findLexSmallestString(self, s: str, a: int, b: int) -> str:
        n = len(s)
        visited = set()
        from collections import deque
        q = deque([s])
        visited.add(s)
        ans = s
        
        while q:
            cur = q.popleft()
            if cur < ans:
                ans = cur
            
            # 操作1
            op1 = list(cur)
            for i in range(1, n, 2):
                op1[i] = str((int(op1[i]) + a) % 10)
            op1_str = ''.join(op1)
            if op1_str not in visited:
                visited.add(op1_str)
                q.append(op1_str)
            
            # 操作2
            op2 = cur[-b:] + cur[:-b]
            if op2 not in visited:
                visited.add(op2)
                q.append(op2)

        return ans

# 方法二：数学 + 法转轮
class Solution:
    def findLexSmallestString(self, s: str, a: int, b: int) -> str:
        from math import gcd
        n = len(s)
        g = gcd(a, 10)
        step = gcd(n, b)
        ans = s
        def change(t: list, start: int):
            ch = int(t[start])
            inc = (ch % g + 10 - ch) % 10
            for j in range(start, n, 2):
                t[j] = str((int(t[j]) + inc) % 10)
        for i in range(0, n, step):
            # 轮转
            t = list(s[i:] + s[:i])
            change(t, 1)  # 调整奇数位
            if step & 1:
                change(t, 0)  # 调整偶数位
            candidate = ''.join(t)
            if candidate < ans:
                ans = candidate
        return ans