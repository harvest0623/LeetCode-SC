class Solution {
public:
    pair<TreeNode*, int> dfs(TreeNode* root) {
        if (!root) {
            return {root, 0};
        }
        auto left = dfs(root->left);
        auto right = dfs(root->right);
        if (left.second > right.second) {
            return {left.first, left.second + 1};
        }
        if (left.second < right.second) {
            return {right.first, right.second + 1};
        }
        return {root, left.second + 1};

    }
    TreeNode* subtreeWithAllDeepest(TreeNode* root) {
        return dfs(root).first;
    }
};