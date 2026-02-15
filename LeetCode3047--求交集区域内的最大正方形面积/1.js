var largestSquareArea = function(bottomLeft, topRight) {
    const n = bottomLeft.length;
    let maxArea = 0;
    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) {
            const x1 = Math.max(bottomLeft[i][0], bottomLeft[j][0]);
            const x2 = Math.min(topRight[i][0], topRight[j][0]);
            const y1 = Math.max(bottomLeft[i][1], bottomLeft[j][1]);
            const y2 = Math.min(topRight[i][1], topRight[j][1]);
            if (x1 < x2 && y1 < y2) {
                const side = Math.min(x2 - x1, y2 - y1);
                maxArea = Math.max(maxArea, side * side);
            }
        }
    }
    return maxArea;
};