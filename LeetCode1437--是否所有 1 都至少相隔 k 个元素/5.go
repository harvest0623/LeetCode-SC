func kLengthApart(nums []int, k int) bool {
    n := len(nums)
    prev := -1
    for i := 0; i < n; i++ {
        if nums[i] == 1 {
            if prev != -1 && i - prev - 1 < k {
                return false
            }
            prev = i
        }
    }
    return true
}