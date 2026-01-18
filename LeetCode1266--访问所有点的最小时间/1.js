var minTimeToVisitAllPoints = function(points) {
    let total = 0;
    let n = points.length;
    for (let i = 1; i < n; i++) {
        const dx = Math.abs(points[i][0] - points[i-1][0]);
        const dy = Math.abs(points[i][1] - points[i-1][1]);
        total += Math.max(dx, dy);
    }
    return total;
};