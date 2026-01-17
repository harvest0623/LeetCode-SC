func prefixesDivBy5(nums []int) []bool {
    ans := make([]bool, len(nums))
    x := 0
    for i, v := range nums {
        x = (x<<1 | v) % 5
        ans[i] = x == 0
    }
    return ans
}