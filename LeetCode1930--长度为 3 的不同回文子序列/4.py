# 方法一：枚举两侧的字符
class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        n = len(s)
        res = 0
        # 枚举两侧字符
        for i in range(26):
            l, r = 0, n - 1
            # 寻找该字符第一次出现的下标
            while l < n and ord(s[l]) - ord('a') != i:
                l += 1
            # 寻找该字符最后一次出现的下标
            while r >= 0 and ord(s[r]) - ord('a') != i:
                r -= 1
            if r - l < 2:
                # 该字符未出现，或两下标中间的子串不存在
                continue
            # 利用哈希集合统计 s[l+1..r-1] 子串的字符总数，并更新答案
            charset = set()
            for k in range(l + 1, r):
                charset.add(s[k])
            res += len(charset)
        return res

# 方法二：枚举中间的字符
class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        n = len(s)
        res = 0
        # 前缀/后缀字符状态数组
        pre = [0] * n
        suf = [0] * n
        for i in range(n):
            # 前缀 s[0..i-1] 包含的字符种类
            pre[i] = (pre[i - 1] if i else 0) | (1 << (ord(s[i]) - ord('a')))
        for i in range(n - 1, -1, -1):
            # 后缀 s[i+1..n-1] 包含的字符种类
            suf[i] = (suf[i + 1] if i != n - 1 else 0) | (1 << (ord(s[i]) - ord('a')))
        # 每种中间字符的回文子序列状态数组
        ans = [0] * 26
        for i in range(1, n - 1):
            ans[ord(s[i]) - ord('a')] |= pre[i - 1] & suf[i + 1]
        # 更新答案
        for i in range(26):
            res += bin(ans[i]).count("1")
        return res