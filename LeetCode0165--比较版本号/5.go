// 方法一：字符串分割
func compareVersion(version1, version2 string) int {
    v1 := strings.Split(version1, ".")
    v2 := strings.Split(version2, ".")
    for i := 0; i < len(v1) || i < len(v2); i++ {
        x, y := 0, 0
        if i < len(v1) {
            x, _ = strconv.Atoi(v1[i])
        }
        if i < len(v2) {
            y, _ = strconv.Atoi(v2[i])
        }
        if x > y {
            return 1
        }
        if x < y {
            return -1
        }
    }
    return 0
}

// 方法二：双指针
func compareVersion(version1, version2 string) int {
    n, m := len(version1), len(version2)
    i, j := 0, 0
    for i < n || j < m {
        x := 0
        for ; i < n && version1[i] != '.'; i++ {
            x = x*10 + int(version1[i]-'0')
        }
        i++ // 跳过点号
        y := 0
        for ; j < m && version2[j] != '.'; j++ {
            y = y*10 + int(version2[j]-'0')
        }
        j++ // 跳过点号
        if x > y {
            return 1
        }
        if x < y {
            return -1
        }
    }
    return 0
}