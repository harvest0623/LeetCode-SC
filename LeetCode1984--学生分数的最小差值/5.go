func minimumDifference(nums []int, k int) int {
    if k == 1 {
        return 0
    }
    sort.Ints(nums)
    ans := math.MaxInt32
    for i := 0; i <= len(nums)-k; i++ {
        diff := nums[i+k-1] - nums[i]
        if diff < ans {
            ans = diff
        }
    }
    return ans
}