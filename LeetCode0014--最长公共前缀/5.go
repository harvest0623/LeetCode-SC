func longestCommonPrefix(strs []string) string {
    s0 := strs[0]
    for j, c := range s0 { 
        for _, s := range strs {
            if j == len(s) || s[j] != byte(c) { 
                return s0[:j] 
            }
        }
    }
    return s0
}