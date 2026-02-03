var countCollisions = function(s) { 
    const n = s.length;
    // 前缀向左的车不会发生碰撞
    let l = 0;
    while (l < n && s[l] === 'L') {
        l++;
    }
    // 后缀向右的车不会发生碰撞
    let r = n;
    while (r > l && s[r - 1] === 'R') {
        r--;
    }
    // 剩下非静止的车计入碰撞次数
    let cnt = 0; 
    for (let i = l; i < r; i++) {
        if (s[i] !== 'S') {
            cnt++;
        }
    }
    return cnt;
};