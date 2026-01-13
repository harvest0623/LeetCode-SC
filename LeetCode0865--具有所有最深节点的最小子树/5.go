func subtreeWithAllDeepest(root *TreeNode) *TreeNode {
    _, lca := dfs(root)
    return lca
}
func dfs(root *TreeNode) (int, *TreeNode) {
    if root == nil {
        return 0, nil
    }
    d1, lca1 := dfs(root.Left)
    h2, lca2 := dfs(root.Right)
    if d1 > h2 {
        return d1 + 1, lca1
    }
    if d1 < h2 {
        return h2 + 1, lca2
    }
    return d1 + 1, root
}