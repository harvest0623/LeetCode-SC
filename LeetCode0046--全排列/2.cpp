class Solution {
public:
    void back(vector<vector<int>>& res, vector<int>& ans, int left, int len) {
        if (left == len) {
            res.emplace_back(ans);
            return;
        }
        for (int i = left; i < len; ++i) {
            swap(ans[i], ans[left]);
            back(res, ans, left + 1, len);
            swap(ans[i], ans[left]);
        }
    }
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> res;
        back(res, nums, 0, (int)nums.size());
        return res;
    }
};