// 方法一：排序
class Solution {
public:
    int minimumCost(vector<int>& nums) {
        sort(nums.begin() + 1, nums.end());
        return reduce(nums.begin(), nums.begin() + 3, 0);
    }
};

// 方法二：维护最小值和次小值
class Solution {
public:
    int minimumCost(vector<int> &nums) {
        int n1 = INT_MAX, n2 = INT_MAX;
        for (int i = 1; i < nums.size(); i++) {
            int x = nums[i];
            if (x < n1) {
                n2 = n1;
                n1 = x;
            } else if (x < n2) {
                n2 = x;
            }
        }
        return nums[0] + n1 + n2;
    }
};