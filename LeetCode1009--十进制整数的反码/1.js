var bitwiseComplement = function(n) {
    let highbit = 0;
    for (let i = 1; i <= 30; ++i) {
        if (n >= 1 << i) {
            highbit = i;
        } else {
            break;
        }            
    }
    const mask = highbit == 30 ? 0x7fffffff : (1 << (highbit + 1)) - 1;
    return n ^ mask;
};