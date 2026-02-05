// 左右上下
const DIRS = [[0, -1], [0, 1], [-1, 0], [1, 0]];

var countUnguarded = function(m, n, guards, walls) {
    const guarded = Array.from({ length: m }, () => Array(n).fill(0));

    // 标记警卫格子、墙格子
    for (const [x, y] of guards) {
        guarded[x][y] = -1;
    }
    for (const [x, y] of walls) {
        guarded[x][y] = -1;
    }

    // 遍历警卫
    for (const [x0, y0] of guards) {
        // 遍历视线方向（左右上下）
        for (const [dx, dy] of DIRS) {
            // 视线所及之处，被保卫
            let x = x0 + dx, y = y0 + dy;
            while (0 <= x && x < m && 0 <= y && y < n && guarded[x][y] !== -1) {
                guarded[x][y] = 1; // 被保卫
                x += dx;
                y += dy;
            }
        }
    }

    // 统计没被保卫的格子数
    let ans = 0;
    for (const row of guarded) {
        for (const x of row) {
            if (x === 0) { // 没被保卫
                ans++;
            }
        }
    }
    return ans;
};