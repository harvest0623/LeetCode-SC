// 方法一：哈希表
func getSneakyNumbers(nums []int) []int {
    res := []int{}
    cnt := make(map[int]int)
    for _, x := range nums {
        cnt[x]++
        if cnt[x] == 2 {
            res = append(res, x)
        }
    }
    return res
}

// 方法二：位运算
func getSneakyNumbers(nums []int) []int {
    n := len(nums) - 2
    y := 0
    for _, x := range nums {
        y ^= x
    }
    for i := 0; i < n; i++ {
        y ^= i
    }
    lowBit := y & -y
    x1, x2 := 0, 0
    for _, x := range nums {
        if x&lowBit != 0 {
            x1 ^= x
        } else {
            x2 ^= x
        }
    }
    for i := 0; i < n; i++ {
        if i&lowBit != 0 {
            x1 ^= i
        } else {
            x2 ^= i
        }
    }
    return []int{x1, x2}
}