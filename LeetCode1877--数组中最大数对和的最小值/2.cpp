class Solution {
public:
    int minPairSum(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        int n = nums.size();
        int maxSum = 0;
        for (int i = 0; i < n / 2; i++) {
            maxSum = max(maxSum, nums[i] + nums[n - 1 - i]);
        }
        return maxSum;
    }
};