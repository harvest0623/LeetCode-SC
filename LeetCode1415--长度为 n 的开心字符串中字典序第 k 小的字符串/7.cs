public class Solution {
    public string GetHappyString(int n, int k) {
        char[] chs = { 'a', 'b', 'c' };
        StringBuilder res = new StringBuilder();
        if (k > 3 * (1 << (n - 1))) {
            return res.ToString();
        }
        for (int i = 0; i < n; i++) {
            int count = 1 << (n - i - 1);
            foreach (char c in chs) {
                if (res.Length > 0 && res[res.Length - 1] == c) {
                    continue;
                }
                if (k <= count) {
                    res.Append(c);
                    break;
                }
                k -= count;
            }
        }
        return res.ToString();
    }
}