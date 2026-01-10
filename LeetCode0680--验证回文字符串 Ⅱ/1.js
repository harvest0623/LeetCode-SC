function checkPalindrome(s, low, high) {
    while (low < high) {
        if (s[low] !== s[high]) {
            return false;
        }
        low++;
        high--;
    }
    return true;
}

function validPalindrome(s) {
    let low = 0, high = s.length - 1;
    while (low < high) {
        if (s[low] === s[high]) {
            low++;
            high--;
        } else {
            return checkPalindrome(s, low, high - 1) || checkPalindrome(s, low + 1, high);
        }
    }
    return true;
}