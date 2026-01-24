func minCost(colors string, neededTime []int) int {
    i, n := 0, len(colors)
    ret := 0
    for i < n {
        ch := colors[i]
        maxValue := 0
        sum := 0
        for i < n && colors[i] == ch {
            if neededTime[i] > maxValue {
                maxValue = neededTime[i]
            }
            sum += neededTime[i]
            i++
        }
        ret += sum - maxValue
    }
    return ret
}