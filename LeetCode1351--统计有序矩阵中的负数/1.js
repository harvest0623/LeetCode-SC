// 方法一：遍历 (O(m × n))
var countNegatives = function(grid) {
    let cnt = 0;
    for (let row of grid) {
        for (let val of row) {
            if (val < 0) cnt++;
        }
    }
    return cnt;
};

// 方法二：有序性 (O(m + n))
var countNegatives = function(grid) {
    const m = grid.length, n = grid[0].length;
    let r = 0, c = n - 1;
    let cnt = 0;
    while (r < m && c >= 0) {
        if (grid[r][c] < 0) {
            cnt += m - r;
            c--;
        } else {
            r++;
        }
    }
    return cnt;
};