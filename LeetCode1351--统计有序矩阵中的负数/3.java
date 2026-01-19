// 方法一：遍历 (O(m × n))
class Solution {
    public int countNegatives(int[][] grid) {
        int cnt = 0;
        for (int[] row : grid) {
            for (int val : row) {
                if (val < 0) cnt++;
            }
        }
        return cnt;
    }
}

// 方法二：有序性 (O(m + n))
class Solution {
    public int countNegatives(int[][] grid) {
        int m = grid.length, n = grid[0].length;
        int r = 0, c = n - 1;
        int cnt = 0;
        while (r < m && c >= 0) {
            if (grid[r][c] < 0) {
                cnt += m - r;
                c--;
            } else {
                r++;
            }
        }
        return cnt;
    }
}