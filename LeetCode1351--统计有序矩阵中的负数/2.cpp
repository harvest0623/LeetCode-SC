// 方法一：遍历 (O(m × n))
class Solution {
public:
    int countNegatives(vector<vector<int>>& grid) {
        int cnt = 0;
        for (const auto& row : grid) {
            for (int val : row) {
                if (val < 0) cnt++;
            }
        }
        return cnt;
    }
};

// 方法二：有序性 (O(m + n))
class Solution {
public:
    int countNegatives(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        int r = 0, c = n - 1; // 从右上角开始
        int cnt = 0;
        while (r < m && c >= 0) {
            if (grid[r][c] < 0) {
                cnt += m - r; // 当前列下面都是负数
                c--; // 左移一列
            } else {
                r++; // 当前非负，则向下移一行
            }
        }
        return cnt;
    }
};