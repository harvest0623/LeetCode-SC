function minFlips(s: string): number {
    // 示性函数
    const I = (ch: string, x: number): number => parseInt(ch) === x ? 1 : 0;
    
    const n = s.length;
    const pre: number[][] = new Array(n);
    for (let i = 0; i < n; i++) {
        pre[i] = new Array(2);
    }
    
    // 注意 i=0 的边界情况
    for (let i = 0; i < n; ++i) {
        if (i === 0) {
            pre[i][0] = I(s[i], 1);
            pre[i][1] = I(s[i], 0);
        } else {
            pre[i][0] = pre[i - 1][1] + I(s[i], 1);
            pre[i][1] = pre[i - 1][0] + I(s[i], 0);
        }
    }
    
    let ans = Math.min(pre[n - 1][0], pre[n - 1][1]);
    if (n % 2 === 1) {
        // 如果 n 是奇数，还需要求出 suf
        const suf: number[][] = new Array(n);
        for (let i = 0; i < n; i++) {
            suf[i] = new Array(2);
        }
        
        // 注意 i=n-1 的边界情况
        for (let i = n - 1; i >= 0; --i) {
            if (i === n - 1) {
                suf[i][0] = I(s[i], 1);
                suf[i][1] = I(s[i], 0);
            } else {
                suf[i][0] = suf[i + 1][1] + I(s[i], 1);
                suf[i][1] = suf[i + 1][0] + I(s[i], 0);
            }
        }
        
        for (let i = 0; i + 1 < n; ++i) {
            ans = Math.min(ans, pre[i][0] + suf[i + 1][0]);
            ans = Math.min(ans, pre[i][1] + suf[i + 1][1]);
        }
    }
    return ans;
}