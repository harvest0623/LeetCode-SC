func maxSideLength(mat [][]int, threshold int) int {
    m, n := len(mat), len(mat[0])
    // 创建二维前缀和数组
    prefix := make([][]int, m+1)
    for i := range prefix {
        prefix[i] = make([]int, n+1)
    }
    
    // 计算二维前缀和
    for i := 1; i <= m; i++ {
        for j := 1; j <= n; j++ {
            prefix[i][j] = prefix[i-1][j] + prefix[i][j-1] - prefix[i-1][j-1] + mat[i-1][j-1]
        }
    }
    
    // 二分查找
    left, right := 0, min(m, n)
    ans := 0
    
    for left <= right {
        mid := left + (right-left)/2
        found := false
        
        // 检查所有边长为mid的正方形
        for i := 1; i+mid <= m+1; i++ {
            for j := 1; j+mid <= n+1; j++ {
                sum := prefix[i+mid-1][j+mid-1] - prefix[i-1][j+mid-1] - 
                      prefix[i+mid-1][j-1] + prefix[i-1][j-1]
                if sum <= threshold {
                    found = true
                    break
                }
            }
            if found {
                break
            }
        }        
        if found {
            ans = mid
            left = mid + 1
        } else {
            right = mid - 1
        }
    }   
    return ans
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}