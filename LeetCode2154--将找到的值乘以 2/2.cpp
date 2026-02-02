// 方法一：排序
class Solution {
public:
    int findFinalValue(vector<int>& nums, int original) {
        sort(nums.begin(), nums.end());
        for (int num: nums) {
            if (original == num) {
                original *= 2;
            }
        }
        return original;
    }
};

// 方法二：哈希表
class Solution {
public:
    int findFinalValue(vector<int>& nums, int original) {
        unordered_set<int> s(nums.begin(), nums.end());
        while (s.count(original)) {
            original *= 2;
        }
        return original;
    }
};