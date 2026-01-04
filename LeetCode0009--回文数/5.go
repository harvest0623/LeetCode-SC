func isPalindrome(x int) bool {
    if x < 0 || x > 0 && x%10 == 0 {
        return false
    }
    rev := 0
    for rev < x/10 {
        rev = rev*10 + x%10
        x /= 10
    }
    return rev == x || rev == x/10
}