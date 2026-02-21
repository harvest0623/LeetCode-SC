func isPrime(x int) bool {
    if x < 2 return false
    for i := 2; i*i <= x; i++ {
        if x%i == 0 return false
    }
    return true
}

func countPrimeSetBits(left, right int) (ans int) {
    for x := left; x <= right; x++ {
        if isPrime(bits.OnesCount(uint(x))) ans++
    }
    return
}