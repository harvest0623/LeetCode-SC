// 方法一：排序
func minimumCost(nums []int) int {
    sort.Ints(nums[1:])
    return nums[0] + nums[1] + nums[2]
}

// 方法二：维护最小值和次小值
func minimumCost(nums []int) int {
    n1 := int(^uint(0) >> 1)
    n2 := int(^uint(0) >> 1)
    for i := 1; i < len(nums); i++ {
        x := nums[i]
        if x < n1 {
            n2 = n1
            n1 = x
        } else if x < n2 {
            n2 = x
        }
    }
    return nums[0] + n1 + n2
}