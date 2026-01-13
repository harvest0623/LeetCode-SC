var subtreeWithAllDeepest = function(root) {
    return dfs(root)[1];
};
function dfs(root) {
    if (!root) {
      return [0, root];
    }
    let [d1, lca1] = dfs(root.left);
    let [d2, lca2] = dfs(root.right);
    if (d1 > d2) {
      return [d1 + 1, lca1];
    }
    if (d1 < d2) {
      return [d2 + 1, lca2];
    }
    return [d1 + 1, root];
}