var minimumCost = function(source, target, original, changed, cost) {
    const g = Array.from({ length: 26 }, () => new Array(26).fill(0x3f3f3f3f));
    for (let i = 0; i < 26; i++) g[i][i] = 0;
    for (let i = 0; i < original.length; i++) {
        const x = original[i].charCodeAt() - 97;
        const y = changed[i].charCodeAt() - 97;
        g[x][y] = Math.min(g[x][y], cost[i]);
    }
    for (let k = 0; k < 26; k++) {
        for (let i = 0; i < 26; i++) {
            if (g[i][k] >= 0x3f3f3f3f) continue;
            for (let j = 0; j < 26; j++) {
                g[i][j] = Math.min(g[i][j], g[i][k] + g[k][j]);
            }
        }
    }
    let ans = 0;
    for (let i = 0; i < source.length; i++) {
        const a = source.charCodeAt(i) - 97;
        const b = target.charCodeAt(i) - 97;
        if (g[a][b] >= 0x3f3f3f3f) return -1;
        ans += g[a][b];
    }
    return ans;
};