// 方法一：贪心
func maxSumDivThree(nums []int) int {
    total := 0
    mod1, mod2 := []int{}, []int{}
    
    for _, num := range nums {
        total += num
        if num % 3 == 1 {
            mod1 = append(mod1, num)
        } else if num % 3 == 2 {
            mod2 = append(mod2, num)
        }
    }
    
    sort.Ints(mod1)
    sort.Ints(mod2)
    
    if total % 3 == 0 {
        return total
    } else if total % 3 == 1 {
        remove := int(^uint(0) >> 1) // 最大int值
        if len(mod1) >= 1 && mod1[0] < remove {
            remove = mod1[0]
        }
        if len(mod2) >= 2 && mod2[0] + mod2[1] < remove {
            remove = mod2[0] + mod2[1]
        }
        
        if remove == int(^uint(0) >> 1) {
            return 0
        }
        return total - remove
    } else { // total % 3 == 2
        remove := int(^uint(0) >> 1)
        if len(mod2) >= 1 && mod2[0] < remove {
            remove = mod2[0]
        }
        if len(mod1) >= 2 && mod1[0] + mod1[1] < remove {
            remove = mod1[0] + mod1[1]
        }
        
        if remove == int(^uint(0) >> 1) {
            return 0
        }
        return total - remove
    }
}

// 方法二：动态规划
func maxSumDivThree(nums []int) int {
    dp := [3]int{0, -1 << 31, -1 << 31}   
    for _, num := range nums {
        newDp := dp  
        for i := 0; i < 3; i++ {
            if dp[i] != -1 << 31 {
                newSum := dp[i] + num
                newRemainder := newSum % 3
                if newSum > newDp[newRemainder] {
                    newDp[newRemainder] = newSum
                }
            }
        }       
        dp = newDp
    }  
    if dp[0] > 0 {
        return dp[0]
    }
    return 0
}