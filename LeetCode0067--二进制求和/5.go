func addBinary(a, b string) string {
    ans := []byte{}
    i := len(a) - 1 
    j := len(b) - 1
    carry := byte(0) 
    for i >= 0 || j >= 0 || carry > 0 {
        sum := carry
        if i >= 0 {
            sum += a[i] - '0'
        }
        if j >= 0 {
            sum += b[j] - '0'
        }
        ans = append(ans, sum%2+'0')
        carry = sum / 2
        i--
        j--
    }
    slices.Reverse(ans)
    return string(ans)
}