// 方法一：BFS 搜索 + 转轮
var findLexSmallestString = function(s, a, b) {
    const n = s.length;
    const visited = new Set();
    const queue = [s];
    visited.add(s);
    let ans = s;
    
    while (queue.length) {
        const cur = queue.shift();
        if (cur < ans) ans = cur;
        
        // 操作1：奇数位加a
        let op1 = cur.split('');
        for (let i = 1; i < n; i += 2) {
            op1[i] = String((parseInt(op1[i]) + a) % 10);
        }
        const op1Str = op1.join('');
        if (!visited.has(op1Str)) {
            visited.add(op1Str);
            queue.push(op1Str);
        }
        
        // 操作2：轮转b位
        const op2 = cur.slice(n - b) + cur.slice(0, n - b);
        if (!visited.has(op2)) {
            visited.add(op2);
            queue.push(op2);
        }
    }
    return ans;
};

// 方法二：数学 + 法转轮
function gcd(a, b) {
    return b === 0 ? a : gcd(b, a % b);
}

var findLexSmallestString = function(s, a, b) {
    const n = s.length;
    const g = gcd(a, 10);
    const step = gcd(n, b);
    let ans = s;
    for (let i = 0; i < n; i += step) {
        // 轮转
        let t = s.substring(i) + s.substring(0, i);
        t = t.split('');
        const change = (start) => {
            const ch = parseInt(t[start]);
            const inc = (ch % g + 10 - ch) % 10;
            for (let j = start; j < n; j += 2) {
                t[j] = ((parseInt(t[j]) + inc) % 10).toString();
            }
        };
        change(1);  // 调整奇数位
        if (step & 1) {
            change(0);  // 调整偶数位
        }
        const tStr = t.join('');
        if (tStr < ans) {
            ans = tStr;
        }
    }
    return ans;
};