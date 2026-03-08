function minOperations(s, k) {
    const n = s.length;
    const z = s.split('').filter(c => c === '0').length;
    
    if (z === 0) {
        return 0;
    }
    if (k === n) {
        return z === n ? 1 : -1;
    }
    let ans = Infinity;
    
    // 情况一：操作次数 m 是偶数
    if (z % 2 === 0) {
        const m = Math.max(
            Math.ceil(z / k),
            Math.ceil(z / (n - k))
        );
        ans = m + (m % 2); // 把 m 往上调整为偶数
    }
    
    // 情况二：操作次数 m 是奇数
    if (z % 2 === k % 2) {
        const m = Math.max(
            Math.ceil(z / k),
            Math.ceil((n - z) / (n - k))
        );
        const oddM = m % 2 === 1 ? m : m + 1; // 把 m 往上调整为奇数
        ans = Math.min(ans, oddM);
    }
    return ans < Infinity ? ans : -1;
}