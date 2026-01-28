var latestDayToCross = function(row, col, cells) {
    const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
        
    let left = 0, right = row * col, ans = 0;
    while (left <= right) {
        const mid = Math.floor((left + right) / 2);
        const grid = Array.from({length: row}, () => 
            Array.from({length: col}, () => 1));
        for (let i = 0; i < mid; i++) {
            grid[cells[i][0] - 1][cells[i][1] - 1] = 0;
        }
        
        const queue = [];
        for (let i = 0; i < col; i++) {
            if (grid[0][i] === 1) {
                queue.push([0, i]);
                grid[0][i] = 0;
            }
        }
        
        let found = false;
        while (queue.length > 0) {
            const [x, y] = queue.shift();
            for (const [dx, dy] of dirs) {
                const nx = x + dx;
                const ny = y + dy;
                if (nx >= 0 && nx < row && ny >= 0 && ny < col && grid[nx][ny] === 1) {
                    if (nx === row - 1) {
                        found = true;
                        break;
                    }
                    queue.push([nx, ny]);
                    grid[nx][ny] = 0;
                }
            }
            if (found) break;
        }
        
        if (found) {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return ans;
};