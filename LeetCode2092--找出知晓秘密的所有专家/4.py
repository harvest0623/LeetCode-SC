class Solution:
    def findAllPeople(self, n: int, meetings: List[List[int]], firstPerson: int) -> List[int]:
        m = len(meetings)
        meetings.sort(key=lambda x: x[2])

        secret = [False] * n
        secret[0] = secret[firstPerson] = True

        i = 0
        while i < m:
            # meetings[i .. j] 为同一时间
            j = i
            while j + 1 < m and meetings[j + 1][2] == meetings[i][2]:
                j += 1

            vertices = set()
            edges = defaultdict(list)
            for k in range(i, j + 1):
                x, y = meetings[k][0], meetings[k][1]
                vertices.update([x, y])
                edges[x].append(y)
                edges[y].append(x)
            
            q = deque([u for u in vertices if secret[u]])
            while q:
                u = q.popleft()
                for v in edges[u]:
                    if not secret[v]:
                        secret[v] = True
                        q.append(v)
            
            i = j + 1
        
        ans = [i for i in range(n) if secret[i]]
        return ans