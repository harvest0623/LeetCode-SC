func findRotation(mat [][]int, target [][]int) bool {
    n := len(mat)
    // 最多旋转 4 次
    for k := 0; k < 4; k++ {
        // 旋转操作
        for i := 0; i < n/2; i++ {
            for j := 0; j < (n+1)/2; j++ {
                mat[i][j], mat[n-1-j][i], mat[n-1-i][n-1-j], mat[j][n-1-i] = 
                    mat[n-1-j][i], mat[n-1-i][n-1-j], mat[j][n-1-i], mat[i][j]
            }
        }
        
        if isEqual(mat, target) {
            return true
        }
    }
    return false
}

func isEqual(mat, target [][]int) bool {
    n := len(mat)
    for i := 0; i < n; i++ {
        for j := 0; j < n; j++ {
            if mat[i][j] != target[i][j] {
                return false
            }
        }
    }
    return true
}