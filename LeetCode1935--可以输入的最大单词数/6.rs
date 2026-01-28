use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken: HashSet<char> = broken_letters.chars().collect();   // 无法输入的字符集合
        let mut res = 0;   // 可以完全输入的单词数目
        let mut flag = true;   // 当前字符所在单词是否可被完全输入
        for ch in text.chars() {
            if ch == ' ' {
                // 当前字符为空格，检查上一个单词状态，更新数目并初始化 flag
                if flag {
                    res += 1;
                }
                flag = true;
            } else if broken.contains(&ch) {
                // 当前字符不可被输入，所在单词无法被完全输入，更新 flag
                flag = false;
            }
        }
        // 判断最后一个单词状态并更新数目
        if flag {
            res += 1;
        }
        res
    }
}