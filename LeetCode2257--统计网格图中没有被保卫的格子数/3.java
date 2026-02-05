class Solution {
    // 左右上下
    private static final int[][] DIRS = {{0, -1}, {0, 1}, {-1, 0}, {1, 0}};

    public int countUnguarded(int m, int n, int[][] guards, int[][] walls) {
        int[][] guarded = new int[m][n];

        // 标记警卫格子、墙格子
        for (int[] g : guards) {
            guarded[g[0]][g[1]] = -1;
        }
        for (int[] w : walls) {
            guarded[w[0]][w[1]] = -1;
        }

        // 遍历警卫
        for (int[] g : guards) {
            // 遍历视线方向（左右上下）
            for (int[] d : DIRS) {
                int dx = d[0], dy = d[1];
                // 视线所及之处，被保卫
                int x = g[0] + dx, y = g[1] + dy;
                while (0 <= x && x < m && 0 <= y && y < n && guarded[x][y] != -1) {
                    guarded[x][y] = 1; // 被保卫
                    x += dx;
                    y += dy;
                }
            }
        }

        // 统计没被保卫的格子数
        int ans = 0;
        for (int[] row : guarded) {
            for (int x : row) {
                if (x == 0) { // 没被保卫
                    ans++;
                }
            }
        }
        return ans;
    }
}