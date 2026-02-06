class Solution {
    private static final int MOD = 1_000_000_007;

    public int numberOfPaths(int[][] grid, int k) {
        int m = grid.length;
        int n = grid[0].length;
        int[][][] memo = new int[m][n][k];
        for (int[][] mat : memo) {
            for (int[] row : mat) {
                Arrays.fill(row, -1); // -1 表示没有计算过
            }
        }
        return dfs(m - 1, n - 1, 0, memo, grid, k);
    }

    private int dfs(int i, int j, int s, int[][][] memo, int[][] grid, int k) {
        if (i < 0 || j < 0) { // 出界
            return 0;
        }
        int preS = ((s - grid[i][j]) % k + k) % k; // 保证模 k 结果非负
        if (i == 0 && j == 0) { // 起点
            return preS == 0 ? 1 : 0; // preS == 0 说明 s == grid[i][j] % k
        }
        if (memo[i][j][s] != -1) { // 之前计算过
            return memo[i][j][s];
        }
        int res1 = dfs(i - 1, j, preS, memo, grid, k);
        int res2 = dfs(i, j - 1, preS, memo, grid, k);
        return memo[i][j][s] = (res1 + res2) % MOD;
    }
}