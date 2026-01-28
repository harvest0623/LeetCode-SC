class Solution:
    def canBeTypedWords(self, text: str, brokenLetters: str) -> int:
        broken = set(brokenLetters)   # 无法输入的字符集合
        res = 0   # 可以完全输入的单词数目
        flag = True   # 当前字符所在单词是否可被完全输入
        for ch in text:
            if ch == ' ':
                # 当前字符为空格，检查上一个单词状态，更新数目并初始化 flag
                if flag:
                    res += 1
                flag = True
            elif ch in broken:
                # 当前字符不可被输入，所在单词无法被完全输入，更新 flag
                flag = False
        # 判断最后一个单词状态并更新数目
        if flag:
            res += 1
        return res