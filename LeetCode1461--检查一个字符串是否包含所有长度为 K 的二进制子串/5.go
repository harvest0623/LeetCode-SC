func hasAllCodes(s string, k int) bool {
    if len(s) < (1 << k) + k - 1 {
        return false
    }
    exists := make(map[string]bool)
    for i := 0; i + k <= len(s); i++ {
        substring := s[i:i+k]
        exists[substring] = true
    }
    return len(exists) == (1 << k)
}