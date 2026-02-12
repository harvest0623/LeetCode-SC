class TrieNode {
    TrieNode[] son = new TrieNode[26];
    int sid = -1;
}
class Solution {
    public long minimumCost(String source, String target, String[] original, String[] changed, int[] cost) {
        TrieNode root = new TrieNode();
        int[] sid = new int[]{0};       
        java.util.function.Function<String, Integer> getId = s -> {
            TrieNode node = root;
            for (int i = 0; i < s.length(); i++) {
                int idx = s.charAt(i) - 'a';
                if (node.son[idx] == null) {
                    node.son[idx] = new TrieNode();
                }
                node = node.son[idx];
            }
            if (node.sid < 0) {
                node.sid = sid[0]++;
            }
            return node.sid;
        };        
        // 初始化距离矩阵
        int m = cost.length;
        int INF = Integer.MAX_VALUE / 2;
        int[][] dis = new int[m * 2][m * 2];        
        for (int i = 0; i < m * 2; i++) {
            for (int j = 0; j < m * 2; j++) {
                dis[i][j] = INF;
            }
            dis[i][i] = 0;
        }        
        for (int i = 0; i < m; i++) {
            int x = getId.apply(original[i]);
            int y = getId.apply(changed[i]);
            dis[x][y] = Math.min(dis[x][y], cost[i]);
        }        
        // Floyd 算法求任意两点最短路
        for (int k = 0; k < sid[0]; k++) {
            for (int i = 0; i < sid[0]; i++) {
                if (dis[i][k] == INF) continue;
                for (int j = 0; j < sid[0]; j++) {
                    dis[i][j] = Math.min(dis[i][j], dis[i][k] + dis[k][j]);
                }
            }
        }       
        int n = source.length();
        long[] f = new long[n + 1];
        f[n] = 0;
        long INF_LONG = Long.MAX_VALUE / 2;        
        for (int i = n - 1; i >= 0; i--) {
            // 不修改 source[i]
            f[i] = source.charAt(i) == target.charAt(i) ? f[i + 1] : INF_LONG;            
            TrieNode p = root;
            TrieNode q = root;            
            for (int j = i; j < n; j++) {
                int pIdx = source.charAt(j) - 'a';
                int qIdx = target.charAt(j) - 'a';               
                p = p.son[pIdx];
                q = q.son[qIdx];                
                if (p == null || q == null) break;          
                if (p.sid < 0 || q.sid < 0) continue;          
                // 修改从 i 到 j 的这一段
                int d = dis[p.sid][q.sid];
                if (d < INF) {
                    f[i] = Math.min(f[i], d + f[j + 1]);
                }
            }
        }
        return f[0] < INF_LONG ? f[0] : -1;
    }
}