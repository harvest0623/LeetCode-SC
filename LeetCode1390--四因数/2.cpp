class Solution {
public:
    int sumFourDivisors(vector<int>& nums) {
        int res = 0;
        for (int n : nums) {
            int cnt = 0, sum = 0;
            for (int i = 1; i * i <= n; ++i) {
                if (n % i == 0) {
                    ++cnt;
                    sum += i;
                    if (i * i != n) {
                        ++cnt;
                        sum += n / i;
                    }
                }
            }
            if (cnt == 4)
                res += sum;
        }
        return res;
    }
};