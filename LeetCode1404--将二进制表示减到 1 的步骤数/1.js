var numSteps = function(s) {
    let ans = s.length - 1;
    let i = s.lastIndexOf('1');
    if (i > 0) {
        ans += (s.substring(1, i).split('0').length - 1) + 2;
    }
    return ans;
};