var longestPalindrome = function(s) {
    const len = s.length;
    if (len < 2) return s; // 长度小于2，直接返回原串

    let maxLen = 1; // 初始化最大长度
    let begin = 0;  // 初始化起始位置
    const dp = Array.from({ length: len }, () => Array(len).fill(false));

    // 所有长度为1的子串都是回文串
    for (let i = 0; i < len; i++) {
        dp[i][i] = true;
    }

    const charArray = s.split('');

    // 枚举子串结束位置 j
    for (let j = 1; j < len; j++) {
        // 枚举子串起始位置 i（必须 i < j）
        for (let i = 0; i < j; i++) {
            if (charArray[i] !== charArray[j]) {
                dp[i][j] = false;
            } else {
                if (j - i < 3) {
                    dp[i][j] = true; // 长度为2或3，只要两端相等就是回文
                } else {
                    dp[i][j] = dp[i + 1][j - 1]; // 否则依赖内部是否回文
                }
            }

            // 更新最长回文子串的位置和长度
            if (dp[i][j] && j - i + 1 > maxLen) {
                maxLen = j - i + 1;
                begin = i;
            }
        }
    }
    return s.substring(begin, begin + maxLen);
};