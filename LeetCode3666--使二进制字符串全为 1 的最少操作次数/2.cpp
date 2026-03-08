class Solution {
public:
    int minOperations(string s, int k) {
        int n = s.size();
        int z = ranges::count(s, '0');
        if (z == 0) {
            return 0;
        }
        if (k == n) {
            return z == n ? 1 : -1;
        }
        int ans = INT_MAX;
        
        // 情况一：操作次数 m 是偶数
        if (z % 2 == 0) { // z 必须是偶数
            int m = max((z + k - 1) / k, (z + n - k - 1) / (n - k)); // 下界
            ans = m + m % 2; // 把 m 往上调整为偶数
        }

        // 情况二：操作次数 m 是奇数
        if (z % 2 == k % 2) { // z 和 k 的奇偶性必须相同
            int m = max((z + k - 1) / k, (n - z + n - k - 1) / (n - k)); // 下界
            ans = min(ans, m | 1); // 把 m 往上调整为奇数
        }
        return ans < INT_MAX ? ans : -1;
    }
};