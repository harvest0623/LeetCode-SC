func maxDotProduct(nums1 []int, nums2 []int) int {
    m := len(nums1)
    n := len(nums2)
    f := make([][]int, m)
    for i := range f {
        f[i] = make([]int, n)
    }
    for i := 0; i < m; i++ {
        for j := 0; j < n; j++ {
            xij := nums1[i] * nums2[j]
            f[i][j] = xij
            if i > 0 {
                f[i][j] = max(f[i][j], f[i-1][j])
            }
            if j > 0 {
                f[i][j] = max(f[i][j], f[i][j-1])
            }
            if i > 0 && j > 0 {
                f[i][j] = max(f[i][j], f[i-1][j-1] + xij)
            }
        }
    }
    return f[m-1][n-1]
}