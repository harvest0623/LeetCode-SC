var canBeTypedWords = function(text, brokenLetters) {
    const broken = new Set();   // 无法输入的字符集合
    for (const ch of brokenLetters) {
        broken.add(ch);
    }
    let res = 0;   // 可以完全输入的单词数目
    let flag = true;   // 当前字符所在单词是否可被完全输入
    for (const ch of text) {
        if (ch === ' ') {
            // 当前字符为空格，检查上一个单词状态，更新数目并初始化 flag
            if (flag) {
                ++res;
            }
            flag = true;
        } else if (broken.has(ch)) {
            // 当前字符不可被输入，所在单词无法被完全输入，更新 flag
            flag = false;
        }
    }
    // 判断最后一个单词状态并更新数目
    if (flag) {
        ++res;
    }
    return res;
};