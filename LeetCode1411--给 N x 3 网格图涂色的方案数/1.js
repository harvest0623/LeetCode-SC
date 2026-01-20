var numOfWays = function(n) {
    const mod = 1e9 + 7;    
    let preA = 6, preB = 6;
    for (let i = 2; i <= n; i++) {
        const new_fi0 = (2 * preA + 2 * preB) % mod;
        const new_fi1 = (2 * preA + 3 * preB) % mod;
        preA = new_fi0;
        preB = new_fi1;
    }
    return (preA + preB) % mod;
};