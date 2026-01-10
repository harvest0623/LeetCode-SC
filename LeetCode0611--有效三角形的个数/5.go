// 方法一:排序 + 二分查找
func triangleNumber(nums []int) (ans int) {
    sort.Ints(nums)
    for i, v := range nums {
        for j := i + 1; j < len(nums); j++ {
            ans += sort.SearchInts(nums[j+1:], v+nums[j])
        }
    }
    return
}

// 方法二：排序 + 双指针
func triangleNumber(nums []int) (ans int) {
    sort.Ints(nums)
    for i, v := range nums {
        k := i
        for j := i + 1; j < len(nums); j++ {
            for k+1 < len(nums) && nums[k+1] < v+nums[j] {
                k++
            }
            ans += max(k-j, 0)
        }
    }
    return
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
