var minOperations = function(s) {
    let cnt = 0;
    for (let i = 0; i < s.length; i++) {
        const c = s[i];
        if (c !== (String.fromCharCode('0'.charCodeAt() + i % 2))) {
            cnt++;
        }
    }
    return Math.min(cnt, s.length - cnt);
};