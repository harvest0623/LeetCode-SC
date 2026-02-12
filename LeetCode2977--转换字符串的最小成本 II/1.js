class TrieNode {
    constructor() {
        this.son = new Array(26).fill(null);
        this.sid = -1;
    }
}
var minimumCost = function(source, target, original, changed, cost) {
    const root = new TrieNode();
    let sid = 0;
    const getId = (s) => {
        let node = root;
        for (let i = 0; i < s.length; i++) {
            const idx = s.charCodeAt(i) - 97;
            if (!node.son[idx]) {
                node.son[idx] = new TrieNode();
            }
            node = node.son[idx];
        }
        if (node.sid < 0) {
            node.sid = sid++;
        }
        return node.sid;
    };
    // 初始化距离矩阵
    const m = cost.length;
    const maxDis = Math.floor(Number.MAX_SAFE_INTEGER / 2);
    const INF = maxDis;
    const dis = Array.from({length: m * 2}, () => new Array(m * 2).fill(INF));
    for (let i = 0; i < m * 2; i++) {
        dis[i][i] = 0;
    }
    for (let i = 0; i < m; i++) {
        const x = getId(original[i]);
        const y = getId(changed[i]);
        dis[x][y] = Math.min(dis[x][y], cost[i]);
    }
    // Floyd 算法求任意两点最短路
    for (let k = 0; k < sid; k++) {
        for (let i = 0; i < sid; i++) {
            if (dis[i][k] === INF) continue;
            for (let j = 0; j < sid; j++) {
                dis[i][j] = Math.min(dis[i][j], dis[i][k] + dis[k][j]);
            }
        }
    }
    const n = source.length;
    const f = new Array(n + 1).fill(0);
    f[n] = 0;
    for (let i = n - 1; i >= 0; i--) {
        // 不修改 source[i]
        f[i] = source[i] === target[i] ? f[i + 1] : INF;  
        let p = root;
        let q = root;      
        for (let j = i; j < n; j++) {
            const pIdx = source.charCodeAt(j) - 97;
            const qIdx = target.charCodeAt(j) - 97;
            p = p.son[pIdx];
            q = q.son[qIdx];
            if (!p || !q) break;        
            if (p.sid < 0 || q.sid < 0) continue;        
            // 修改从 i 到 j 的这一段
            const d = dis[p.sid][q.sid];
            if (d < INF) {
                f[i] = Math.min(f[i], d + f[j + 1]);
            }
        }
    }
    return f[0] < INF ? f[0] : -1;
};