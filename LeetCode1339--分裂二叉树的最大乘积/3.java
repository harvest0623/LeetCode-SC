class Solution {
    private int sum = 0;
    private int best = 0;

    public int maxProduct(TreeNode root) {
        dfs(root);
        dfs2(root);
        return (int)((long)best * (sum - best) % 1000000007);
    }

    private void dfs(TreeNode node) {
        if (node == null) {
            return;
        }
        sum += node.val;
        dfs(node.left);
        dfs(node.right);
    }

    private int dfs2(TreeNode node) {
        if (node == null) {
            return 0;
        }
        int cur = dfs2(node.left) + dfs2(node.right) + node.val;
        if (Math.abs(cur * 2 - sum) < Math.abs(best * 2 - sum)) {
            best = cur;
        }
        return cur;
    }
}