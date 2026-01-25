// 方法一：递归
class Solution {
public:
    int minimumOneBitOperations(int n) {
        if (n == 0) {
            return 0;
        }
        int x = 31 - __builtin_clz(n);
        return (1 << (x + 1)) - 1 - minimumOneBitOperations(n - (1 << x));
    }
};

// 方法二：迭代
class Solution {
public:
    int minimumOneBitOperations(int n) {
        int ans = 0;
        int sign = 1;
        for (int i = 29; i >= 0; --i) {
            if (n & (1 << i)) {
                ans += sign * ((1 << (i + 1)) - 1);
                sign = -sign;
            }
        }
        return ans;
    }
};

// 方法三：格雷码转二进制码
class Solution {
public:
    int minimumOneBitOperations(int n) {
        int ans = 0;
        while (n) {
            ans ^= n;
            n >>= 1;
        }
        return ans;
    }
};