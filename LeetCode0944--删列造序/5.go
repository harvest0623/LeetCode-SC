func minDeletionSize(strs []string) (ans int) {
    for j := range strs[0] {
        for i := 1; i < len(strs); i++ {
            if strs[i-1][j] > strs[i][j] {
                ans++
                break
            }
        }
    }
    return
}