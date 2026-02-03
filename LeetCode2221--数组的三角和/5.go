func triangularSum(nums []int) int {
    current := nums    
    for len(current) > 1 {
        newNums := make([]int, 0, len(current)-1)
        for i := 0; i < len(current)-1; i++ {
            newNums = append(newNums, (current[i] + current[i + 1])%10)
        }
        current = newNums
    }
    return current[0]
}