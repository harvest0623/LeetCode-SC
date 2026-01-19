class Solution {
private:
    int sum = 0;
    int best = 0;

public:
    void dfs(TreeNode* node) {
        if (!node) {
            return;
        }
        sum += node->val;
        dfs(node->left);
        dfs(node->right);
    }

    int dfs2(TreeNode* node) {
        if (!node) {
            return 0;
        }
        int cur = dfs2(node->left) + dfs2(node->right) + node->val;
        if (abs(cur*2 - sum) < abs(best*2 - sum)) {
            best = cur;
        }
        return cur;
    }

    int maxProduct(TreeNode* root) {
        dfs(root);
        dfs2(root);
        return (long long)best * (sum - best) % 1000000007;
    }
};