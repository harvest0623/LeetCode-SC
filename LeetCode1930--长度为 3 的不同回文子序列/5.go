// 方法一：枚举两侧的字符
func countPalindromicSubsequence(s string) int {
    n := len(s)
    res := 0
    // 枚举两侧字符
    for ch := 'a'; ch <= 'z'; ch++ {
        l, r := 0, n-1
        // 寻找该字符第一次出现的下标
        for l < n && rune(s[l]) != ch {
            l++
        }
        // 寻找该字符最后一次出现的下标
        for r >= 0 && rune(s[r]) != ch {
            r--
        }
        if r-l < 2 {
            // 该字符未出现，或两下标中间的子串不存在
            continue
        }
        // 利用哈希集合统计 s[l+1..r-1] 子串的字符总数，并更新答案
        charset := make(map[rune]bool)
        for _, c := range s[l+1:r] {
            charset[c] = true
        }
        res += len(charset)
    }
    return res
}

// 方法二：枚举中间的字符
func countPalindromicSubsequence(s string) int {
    n := len(s)
    res := 0
    // 前缀/后缀字符状态数组
    pre := make([]int, n)
    suf := make([]int, n)
    for i := 0; i < n; i++ {
        // 前缀 s[0..i-1] 包含的字符种类
        if i > 0 {
            pre[i] = pre[i-1]
        }
        pre[i] |= 1 << (s[i] - 'a')
    }
    for i := n - 1; i >= 0; i-- {
        // 后缀 s[i+1..n-1] 包含的字符种类
        if i != n - 1 {
            suf[i] = suf[i + 1]
        }
        suf[i] |= 1 << (s[i] - 'a')
    }
    // 每种中间字符的回文子序列状态数组
    ans := make([]int, 26)
    for i := 1; i < n - 1; i++ {
        ans[s[i] - 'a'] |= (pre[i - 1] & suf[i + 1])
    }
    // 更新答案
    for i := 0; i < 26; i++ {
        res += bits.OnesCount(uint(ans[i]))
    }
    return res
}