func minFlips(s string) int {
    // 示性函数
    I := func(ch byte, x int) int {
        if int(ch - '0') == x {
            return 1
        }
        return 0
    }
    
    n := len(s)
    // 使用切片存储
    pre := make([][2]int, n)
    // 注意 i=0 的边界情况
    for i := 0; i < n; i++ {
        if i == 0 {
            pre[i][0] = I(s[i], 1)
            pre[i][1] = I(s[i], 0)
        } else {
            pre[i][0] = pre[i-1][1] + I(s[i], 1)
            pre[i][1] = pre[i-1][0] + I(s[i], 0)
        }
    }
    
    ans := min(pre[n - 1][0], pre[n - 1][1]);
    // 如果是奇数长度，需要考虑移动操作
    if n % 2 == 1 {
        // 如果 n 是奇数，还需要求出 suf
        suf := make([][2]int, n)
        // 注意 i = n - 1 的边界情况
        for i := n - 1; i >= 0; i-- {
            if i == n - 1 {
                suf[i][0] = I(s[i], 1)
                suf[i][1] = I(s[i], 0)
            } else {
                suf[i][0] = suf[i + 1][1] + I(s[i], 1)
                suf[i][1] = suf[i + 1][0] + I(s[i], 0)
            }
        }
        
        for i := 0; i + 1 < n; i++ {
            ans = min(ans, pre[i][0] + suf[i + 1][0]);
            ans = min(ans, pre[i][1] + suf[i + 1][1]);
        }
    }
    
    return ans;
}