var concatenatedBinary = function(n) {
    let sum = "";
    for (let i = 1; i <= n; i++) {
        sum += (i).toString(2); 
        let t = parseInt(sum, 2);   
        if (t >= 1000000007) {
            t %= 1000000007;
            sum = (t).toString(2);
        }
    }
    return parseInt(sum, 2);
};