var maxProduct = function(root) {
    function dfs1(node) {
        if (node === null) {
            return 0;
        }
        return node.val + dfs1(node.left) + dfs1(node.right);
    }
    function dfs2(node) {
        if (node === null) {
            return 0;
        }
        const s = node.val + dfs2(node.left) + dfs2(node.right);
        ans = Math.max(ans, s * (total - s));
        return s;
    }
    const total = dfs1(root);
    let ans = 0;
    dfs2(root);
    return ans % 1000000007;
};