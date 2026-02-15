func largestSquareArea(bottomLeft [][]int, topRight [][]int) int64 {
    n := len(bottomLeft)
    var maxArea int64 = 0
    for i := 0; i < n; i++ {
        for j := i + 1; j < n; j++ {
            x1 := max(bottomLeft[i][0], bottomLeft[j][0])
            x2 := min(topRight[i][0], topRight[j][0])
            y1 := max(bottomLeft[i][1], bottomLeft[j][1])
            y2 := min(topRight[i][1], topRight[j][1])
            if x1 < x2 && y1 < y2 {
                side := min(x2-x1, y2-y1)
                area := int64(side) * int64(side)
                if area > maxArea {
                    maxArea = area
                }
            }
        }
    }
    return maxArea
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}