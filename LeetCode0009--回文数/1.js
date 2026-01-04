var isPalindrome = function (x) {
    if (x < 0 || x > 0 && x % 10 == 0) {
        return false;
    }
    let rev = 0;
    while (rev < Math.floor(x / 10)) {
        rev = rev * 10 + x % 10;
        x = Math.floor(x / 10);
    }
    return rev == x || rev == Math.floor(x / 10);
};