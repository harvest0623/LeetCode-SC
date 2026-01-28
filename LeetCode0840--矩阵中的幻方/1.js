var numMagicSquaresInside = function(grid) {
    const rows = grid.length;
    const cols = grid[0].length;
    if (rows < 3 || cols < 3) return 0;
    let count = 0;
    for (let r = 0; r <= rows - 3; r++) {
        for (let c = 0; c <= cols - 3; c++) {
            if (isMagicSquare(grid, r, c)) {
                count++;
            }
        }
    }
    return count;
};

function isMagicSquare(grid, r, c) {
    if (grid[r + 1][c + 1] !== 5) return false;
    const seen = new Array(10).fill(false);
    for (let i = r; i < r + 3; i++) {
        for (let j = c; j < c + 3; j++) {
            const num = grid[i][j];
            if (num < 1 || num > 9 || seen[num]) return false;
            seen[num] = true;
        }
    }
    const sumRows = (grid[r][c] + grid[r][c+1] + grid[r][c+2] === 15) &&
                    (grid[r+1][c] + grid[r+1][c+1] + grid[r+1][c+2] === 15) &&
                    (grid[r+2][c] + grid[r+2][c+1] + grid[r+2][c+2] === 15);
    if (!sumRows) return false; 
    const sumCols = (grid[r][c] + grid[r+1][c] + grid[r+2][c] === 15) &&
                    (grid[r][c+1] + grid[r+1][c+1] + grid[r+2][c+1] === 15) &&
                    (grid[r][c+2] + grid[r+1][c+2] + grid[r+2][c+2] === 15);
    if (!sumCols) return false;    
    const sumDiag = (grid[r][c] + grid[r+1][c+1] + grid[r+2][c+2] === 15) &&
                    (grid[r][c+2] + grid[r+1][c+1] + grid[r+2][c] === 15);
    return sumDiag;
}