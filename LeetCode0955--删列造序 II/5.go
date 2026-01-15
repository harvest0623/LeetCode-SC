func minDeletionSize(A []string) int {
    cuts := make([]bool, len(A) - 1)
    ans := 0
    for j := 0; j < len(A[0]); j++ {
        canKeep := true
        for i := 0; i < len(A) - 1; i++ {
            if !cuts[i] && A[i][j] > A[i + 1][j] {
                canKeep = false
                break
            }
        }
        if canKeep {
            for i := 0; i < len(A) - 1; i++ {
                if A[i][j] < A[i + 1][j] {
                    cuts[i] = true
                }
            }
        } else {
            ans++
        }
    }
    return ans
}