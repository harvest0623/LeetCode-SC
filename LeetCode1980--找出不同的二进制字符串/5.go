func findDifferentBinaryString(nums []string) string {
    n := len(nums)
    // 预处理对应整数的哈希集合
    vals := make(map[int]bool)
    for _, num := range nums {
        val, _ := strconv.ParseInt(num, 2, 32)
        vals[int(val)] = true
    }
    // 寻找第一个不在哈希集合中的整数
    val := 0
    for vals[val] {
        val++
    }
    // 将整数转化为二进制字符串返回
    binary := strconv.FormatInt(int64(val), 2)
    // 补齐前导0
    if len(binary) < n {
        binary = strings.Repeat("0", n - len(binary)) + binary
    }
    return binary
}