var minCost = function(colors, neededTime) {
    let i = 0, len = colors.length;
    let ret = 0;
    while (i < len) {
        const ch = colors[i];
        let maxValue = 0;
        let sum = 0;
        while (i < len && colors[i] === ch) {
            maxValue = Math.max(maxValue, neededTime[i]);
            sum += neededTime[i];
            i++;
        }
        ret += sum - maxValue;
    }
    return ret;
};