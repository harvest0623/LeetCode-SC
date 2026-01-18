// 方法一：贪心
class Solution {
public:
    int maxSumDivThree(vector<int>& nums) {
        int total = 0;
        // 分别存储余数为1和2的数字
        vector<int> mod1, mod2;
        
        for (int num : nums) {
            total += num;
            if (num % 3 == 1) mod1.push_back(num);
            else if (num % 3 == 2) mod2.push_back(num);
        }
        
        // 排序方便取最小值
        sort(mod1.begin(), mod1.end());
        sort(mod2.begin(), mod2.end());
        
        if (total % 3 == 0) {
            return total;
        } else if (total % 3 == 1) {
            int remove = INT_MAX;
            // 移除一个余数为1的最小数
            if (mod1.size() >= 1) remove = min(remove, mod1[0]);
            // 移除两个余数为2的最小数
            if (mod2.size() >= 2) remove = min(remove, mod2[0] + mod2[1]);
            
            return total - (remove == INT_MAX ? 0 : remove);
        } else { // total % 3 == 2
            int remove = INT_MAX;
            // 移除一个余数为2的最小数
            if (mod2.size() >= 1) remove = min(remove, mod2[0]);
            // 移除两个余数为1的最小数
            if (mod1.size() >= 2) remove = min(remove, mod1[0] + mod1[1]);
            
            return total - (remove == INT_MAX ? 0 : remove);
        }
    }
};

// 方法二：动态规划
class Solution {
public:
    int maxSumDivThree(vector<int>& nums) {
        vector<int> dp = {0, INT_MIN, INT_MIN};
        for (int num : nums) {
            vector<int> new_dp = dp;
            for (int i = 0; i < 3; i++) {
                if (dp[i] != INT_MIN) {
                    int new_sum = dp[i] + num;
                    int new_remainder = new_sum % 3;
                    new_dp[new_remainder] = max(new_dp[new_remainder], new_sum);
                }
            }
            dp = new_dp;
        }   
        return dp[0] > 0 ? dp[0] : 0;
    }
};