class TrieNode:
    def __init__(self):
        self.son = [None] * 26
        self.sid = -1
class Solution:
    def minimumCost(self, source: str, target: str, original: List[str], changed: List[str], cost: List[int]) -> int:
        root = TrieNode()
        sid = 0
        def get_id(s: str) -> int:
            nonlocal sid
            node = root
            for ch in s:
                idx = ord(ch) - 97
                if not node.son[idx]:
                    node.son[idx] = TrieNode()
                node = node.son[idx]
            if node.sid < 0:
                node.sid = sid
                sid += 1
            return node.sid     
        # 初始化距离矩阵
        m = len(cost)
        INF = 10**18
        dis = [[INF] * (m * 2) for _ in range(m * 2)]        
        for i in range(m * 2):
            dis[i][i] = 0       
        for i in range(m):
            x = get_id(original[i])
            y = get_id(changed[i])
            dis[x][y] = min(dis[x][y], cost[i])       
        # Floyd 算法求任意两点最短路
        for k in range(sid):
            for i in range(sid):
                if dis[i][k] == INF:
                    continue
                for j in range(sid):
                    dis[i][j] = min(dis[i][j], dis[i][k] + dis[k][j])       
        n = len(source)
        f = [0] * (n + 1)        
        for i in range(n - 1, -1, -1):
            # 不修改 source[i]
            f[i] = f[i + 1] if source[i] == target[i] else INF
            p = root
            q = root
            for j in range(i, n):
                p_idx = ord(source[j]) - 97
                q_idx = ord(target[j]) - 97                
                if not p.son[p_idx] or not q.son[q_idx]:
                    break                
                p = p.son[p_idx]
                q = q.son[q_idx]                
                if p.sid < 0 or q.sid < 0:
                    continue                
                # 修改从 i 到 j 的这一段
                d = dis[p.sid][q.sid]
                if d < INF:
                    f[i] = min(f[i], d + f[j + 1])        
        return f[0] if f[0] < INF else -1