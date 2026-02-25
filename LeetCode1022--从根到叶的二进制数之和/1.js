var sumRootToLeaf = function(root) {
    const dfs = (root, val) => {
        if (!root) {
            return 0;
        }
        val = (val << 1) | root.val;
        if (!root.left&& !root.right) {
            return val;
        }
        return dfs(root.left, val) + dfs(root.right, val);
    }
    return dfs(root, 0);
};