var largestMagicSquare = function(grid) {
    const m = grid.length, n = grid[0].length;
    const rowPrefix = Array(m + 1).fill().map(() => Array(n + 1).fill(0));
    const colPrefix = Array(m + 1).fill().map(() => Array(n + 1).fill(0));
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            rowPrefix[i + 1][j + 1] = rowPrefix[i + 1][j] + grid[i][j];
            colPrefix[i + 1][j + 1] = colPrefix[i][j + 1] + grid[i][j];
        }
    }
    const getRowSum = (r, c1, c2) => rowPrefix[r + 1][c2 + 1] - rowPrefix[r + 1][c1];
    const getColSum = (c, r1, r2) => colPrefix[r2 + 1][c + 1] - colPrefix[r1][c + 1];
    for (let k = Math.min(m, n); k > 1; k--) {
        for (let r = 0; r + k <= m; r++) {
            for (let c = 0; c + k <= n; c++) {
                const target = getRowSum(r, c, c + k - 1);
                let ok = true;
                for (let i = r; i < r + k; i++) {
                    if (getRowSum(i, c, c + k - 1) !== target) {
                        ok = false;
                        break;
                    }
                }
                if (!ok) continue;
                for (let j = c; j < c + k; j++) {
                    if (getColSum(j, r, r + k - 1) !== target) {
                        ok = false;
                        break;
                    }
                }
                if (!ok) continue;
                let diag1 = 0, diag2 = 0;
                for (let d = 0; d < k; d++) {
                    diag1 += grid[r + d][c + d];
                    diag2 += grid[r + d][c + k - 1 - d];
                }
                if (diag1 !== target || diag2 !== target) continue;            
                return k;
            }
        }
    }
    return 1;
};