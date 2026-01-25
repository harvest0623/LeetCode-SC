func minimumAbsDifference(arr []int) [][]int {
    sort.Ints(arr)
    minDiff := math.MaxInt32
    // 找出最小差值
    for i := 1; i < len(arr); i++ {
        diff := arr[i] - arr[i-1]
        if diff < minDiff {
            minDiff = diff
        }
    }
    // 收集结果
    res := [][]int{}
    for i := 1; i < len(arr); i++ {
        if arr[i] - arr[i-1] == minDiff {
            res = append(res, []int{arr[i-1], arr[i]})
        }
    }
    return res
}