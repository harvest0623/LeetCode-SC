// 方法一：排序
func findFinalValue(nums []int, original int) int {
    sort.Ints(nums)
    for _, num := range nums {
        if original == num {
            original *= 2
        }
    }
    return original
}

// 方法二：哈希表
func findFinalValue(nums []int, original int) int {
    set := make(map[int]bool)
    for _, num := range nums {
        set[num] = true
    }
    for set[original] {
        original *= 2
    }
    return original
}