func sumFourDivisors(nums []int) int {
    ans := 0
    for _, num := range nums {
        cnt, sum := 0, 0
        for i := 1; i*i <= num; i++ {
            if num%i == 0 {
                cnt++
                sum += i
                if i*i != num {   
                    cnt++
                    sum += num / i
                }
            }
        }
        if cnt == 4 {
            ans += sum
        }
    }
    return ans
}