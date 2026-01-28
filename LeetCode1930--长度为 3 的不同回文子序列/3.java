// 方法一：枚举两侧的字符
class Solution {
    public int countPalindromicSubsequence(String s) {
        int n = s.length();
        int res = 0;
        // 枚举两侧字符
        for (char ch = 'a'; ch <= 'z'; ++ch) {
            int l = 0, r = n - 1;
            // 寻找该字符第一次出现的下标
            while (l < n && s.charAt(l) != ch) {
                ++l;
            }
            // 寻找该字符最后一次出现的下标
            while (r >= 0 && s.charAt(r) != ch) {
                --r;
            }
            if (r - l < 2) {
                // 该字符未出现，或两下标中间的子串不存在
                continue;
            }
            // 利用哈希集合统计 s[l+1..r-1] 子串的字符总数，并更新答案
            Set<Character> charset = new HashSet<>();
            for (int k = l + 1; k < r; ++k) {
                charset.add(s.charAt(k));
            }
            res += charset.size();
        }
        return res;
    }
}

// 方法二：枚举中间的字符
class Solution {
    public int countPalindromicSubsequence(String s) {
        int n = s.length();
        int res = 0;
        // 前缀/后缀字符状态数组
        int[] pre = new int[n];
        int[] suf = new int[n];
        for (int i = 0; i < n; ++i) {
            // 前缀 s[0..i-1] 包含的字符种类
            pre[i] = (i > 0 ? pre[i - 1] : 0) | (1 << (s.charAt(i) - 'a'));
        }
        for (int i = n - 1; i >= 0; --i) {
            // 后缀 s[i+1..n-1] 包含的字符种类
            suf[i] = (i != n - 1 ? suf[i + 1] : 0) | (1 << (s.charAt(i) - 'a'));
        }
        // 每种中间字符的回文子序列状态数组
        int[] ans = new int[26];
        for (int i = 1; i < n - 1; ++i) {
            ans[s.charAt(i) - 'a'] |= (pre[i - 1] & suf[i + 1]);
        }
        // 更新答案
        for (int i = 0; i < 26; ++i) {
            res += Integer.bitCount(ans[i]);
        }
        return res;
    }
}