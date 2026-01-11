func intersectionSizeTwo(intervals [][]int) (ans int) {
    sort.Slice(intervals, func(i, j int) bool {
        a, b := intervals[i], intervals[j]
        return a[0] < b[0] || a[0] == b[0] && a[1] > b[1]
    })
    n, m := len(intervals), 2
    vals := make([][]int, n)
    for i := n - 1; i >= 0; i-- {
        for j, k := intervals[i][0], len(vals[i]); k < m; k++ {
            ans++
            for p := i - 1; p >= 0 && intervals[p][1] >= j; p-- {
                vals[p] = append(vals[p], j)
            }
            j++
        }
    }
    return
}