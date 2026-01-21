var avoidFlood = function(rains) {
    const n = rains.length;
    const fa = Array(n + 1).fill(0).map((_, i) => i);
    function find(x) {
        if (fa[x] !== x) {
            fa[x] = find(fa[x]);
        }
        return fa[x];
    }
    const ans = Array(n).fill(-1);
    const fullDay = new Map();
    for (let i = 0; i < n; i++) {
        const lake = rains[i];
        if (lake === 0) {
            ans[i] = 1; 
            continue;
        }
        const j = fullDay.get(lake);
        if (j !== undefined) {
            const dryDay = find(j + 1);
            if (dryDay >= i) {
                return [];
            }
            ans[dryDay] = lake;
            fa[dryDay] = find(dryDay + 1); 
        }
        fa[i] = i + 1; 
        fullDay.set(lake, i);
    }
    return ans;
};