class Solution {
    public long minimumCost(String source, String target, char[] original, char[] changed, int[] cost) {
        int[][] g = new int[26][26];
        for (int i = 0; i < 26; i++) {
            for (int j = 0; j < 26; j++) {
                if (i == j) g[i][j] = 0;
                else g[i][j] = 0x3f3f3f3f;
            }
        }
        for (int i = 0; i < original.length; i++) {
            int x = original[i] - 'a';
            int y = changed[i] - 'a';
            g[x][y] = Math.min(g[x][y], cost[i]);
        }
        for (int k = 0; k < 26; k++) {
            for (int i = 0; i < 26; i++) {
                if (g[i][k] >= 0x3f3f3f3f) continue;
                for (int j = 0; j < 26; j++) {
                    g[i][j] = Math.min(g[i][j], g[i][k] + g[k][j]);
                }
            }
        }
        long ans = 0;
        for (int i = 0; i < source.length(); i++) {
            int a = source.charAt(i) - 'a';
            int b = target.charAt(i) - 'a';
            if (g[a][b] >= 0x3f3f3f3f) return -1;
            ans += g[a][b];
        }
        return ans;
    }
}