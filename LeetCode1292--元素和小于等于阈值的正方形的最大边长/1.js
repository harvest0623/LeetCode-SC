var maxSideLength = function(mat, threshold) {
    const m = mat.length, n = mat[0].length;
    const prefix = Array(m + 1).fill().map(() => Array(n + 1).fill(0));
    
    // 计算二维前缀和
    for (let i = 1; i <= m; i++) {
        for (let j = 1; j <= n; j++) {
            prefix[i][j] = prefix[i-1][j] + prefix[i][j-1] - prefix[i-1][j-1] + mat[i-1][j-1];
        }
    }
    
    // 二分查找
    let left = 0, right = Math.min(m, n);
    let ans = 0;
    
    while (left <= right) {
        const mid = Math.floor((left + right) / 2);
        let found = false;
        
        // 检查所有边长为mid的正方形
        for (let i = 1; i + mid <= m + 1; i++) {
            for (let j = 1; j + mid <= n + 1; j++) {
                const sum = prefix[i+mid-1][j+mid-1] - prefix[i-1][j+mid-1] 
                          - prefix[i+mid-1][j-1] + prefix[i-1][j-1];
                if (sum <= threshold) {
                    found = true;
                    break;
                }
            }
            if (found) break;
        }
        if (found) {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }  
    return ans;
};