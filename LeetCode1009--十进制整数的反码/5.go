func bitwiseComplement(n int) int {
    highBit := 0
    for i := 1; i <= 30; i++ {
        if n < 1<<i {
            break
        }
        highBit = i
    }
    mask := 1<<(highBit+1) - 1
    return n ^ mask
}