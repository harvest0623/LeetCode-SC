// 方法一：暴力计算
class Solution {
public:
    int totalMoney(int n) {
        int week = 1, day = 1;
        int res = 0;
        for (int i = 0; i < n; ++i) {
            res += week + day - 1;
            ++day;
            if (day == 8) {
                day = 1;
                ++week;
            }
        }
        return res;
    }
};

// 方法二：等差数列求和
class Solution {
public:
    int totalMoney(int n) {
        constexpr int D = 7;
        int w = n / D, r = n % D;
        return (w * D * (w + D) + r * (w * 2 + r + 1)) / 2;
    }
};