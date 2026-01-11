var pyramidTransition = function(bottom, allowed) {
    const T = Array.from({length: 7}, () => new Array(7).fill(0));
    for (const a of allowed) {
        const left = a.charCodeAt(0) - 'A'.charCodeAt(0);
        const right = a.charCodeAt(1) - 'A'.charCodeAt(0);
        const top = a.charCodeAt(2) - 'A'.charCodeAt(0);
        T[left][right] |= 1 << top;
    }
    const seen = new Set();
    const N = bottom.length;
    const A = Array.from({length: N}, () => new Array(N).fill(0));
    for (let i = 0; i < N; i++) {
        A[N-1][i] = bottom.charCodeAt(i) - 'A'.charCodeAt(0);
    }
    const solve = (R, N, i) => {
        if (N === 1 && i === 1) {
            return true;
        } else if (i === N) { 
            if (seen.has(R)) {
                return false;
            }
            seen.add(R);
            return solve(0, N-1, 0);
        } else { 
            const w = T[A[N][i]][A[N][i+1]];
            for (let b = 0; b < 7; b++) {
                if ((w >> b) & 1) {
                    A[N-1][i] = b;
                    if (solve(R * 8 + (b + 1), N, i + 1)) {
                        return true;
                    }
                }
            }
            return false;
        }
    };
    return solve(0, N-1, 0);
};