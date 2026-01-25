// 方法一：BFS 搜索 + 转轮
func findLexSmallestString(s string, a int, b int) string {
    n := len(s)
    visited := make(map[string]bool)
    queue := []string{s}
    visited[s] = true
    ans := s
    
    for len(queue) > 0 {
        cur := queue[0]
        queue = queue[1:]
        if cur < ans {
            ans = cur
        }
        
        // 操作1
        op1 := []byte(cur)
        for i := 1; i < n; i += 2 {
            digit := int(op1[i] - '0')
            digit = (digit + a) % 10
            op1[i] = byte(digit + '0')
        }
        op1Str := string(op1)
        if !visited[op1Str] {
            visited[op1Str] = true
            queue = append(queue, op1Str)
        }
        
        // 操作2
        op2 := cur[n-b:] + cur[:n-b]
        if !visited[op2] {
            visited[op2] = true
            queue = append(queue, op2)
        }
    }
    return ans
}

// 方法二：数学 + 法转轮
func gcd(a, b int) int {
    for b != 0 {
        a, b = b, a%b
    }
    return a
}
func findLexSmallestString(s string, a int, b int) string {
    n := len(s)
    g := gcd(a, 10)
    step := gcd(n, b)
    ans := s
    change := func(t []byte, start int) {
        ch := int(t[start] - '0')
        inc := (ch%g + 10 - ch) % 10
        for j := start; j < n; j += 2 {
            digit := int(t[j]-'0') + inc
            t[j] = byte(digit%10 + '0')
        }
    }
    for i := 0; i < n; i += step {
        // 轮转
        t := []byte(s[i:] + s[:i])
        change(t, 1)  // 调整奇数位
        if step&1 == 1 {
            change(t, 0)  // 调整偶数位
        }
        candidate := string(t)
        if candidate < ans {
            ans = candidate
        }
    }
    return ans
}