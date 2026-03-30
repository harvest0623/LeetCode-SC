public class Answer {
    private int[] ans;
    
    public Answer() {
        ans = new int[3];
    }
    
    public void Put(int x) {
        if (x > ans[0]) {
            ans[2] = ans[1];
            ans[1] = ans[0];
            ans[0] = x;
        }
        else if (x != ans[0] && x > ans[1]) {
            ans[2] = ans[1];
            ans[1] = x;
        }
        else if (x != ans[0] && x != ans[1] && x > ans[2]) {
            ans[2] = x;
        }
    }
    
    public List<int> Get() {
        List<int> ret = new List<int>();
        foreach (int num in ans) {
            if (num != 0) {
                ret.Add(num);
            }
        }
        return ret;
    }
}

public class Solution {
    public int[] GetBiggestThree(int[][] grid) {
        int m = grid.Length, n = grid[0].Length;
        int[,] sum1 = new int[m + 1, n + 2];
        int[,] sum2 = new int[m + 1, n + 2];
        
        for (int i = 1; i <= m; ++i) {
            for (int j = 1; j <= n; ++j) {
                sum1[i, j] = sum1[i - 1, j - 1] + grid[i - 1][j - 1];
                sum2[i, j] = sum2[i - 1, j + 1] + grid[i - 1][j - 1];
            }
        }
        
        Answer ans = new Answer();
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                // 单独的一个格子也是菱形
                ans.Put(grid[i][j]);
                for (int k = i + 2; k < m; k += 2) {
                    int ux = i, uy = j;
                    int dx = k, dy = j;
                    int lx = (i + k) / 2, ly = j - (k - i) / 2;
                    int rx = (i + k) / 2, ry = j + (k - i) / 2;
                    if (ly < 0 || ry >= n) {
                        break;
                    }
                    int sum = (sum2[lx + 1, ly + 1] - sum2[ux, uy + 2]) +
                              (sum1[rx + 1, ry + 1] - sum1[ux, uy]) +
                              (sum1[dx + 1, dy + 1] - sum1[lx, ly]) +
                              (sum2[dx + 1, dy + 1] - sum2[rx, ry + 2]) -
                              (grid[ux][uy] + grid[dx][dy] + grid[lx][ly] + grid[rx][ry]);
                    ans.Put(sum);
                }
            }
        }
        
        List<int> resultList = ans.Get();
        return resultList.ToArray();
    }
}