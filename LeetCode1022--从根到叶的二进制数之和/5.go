func dfs(node *TreeNode, val int) int {
    if node == nil {
        return 0
    }
    val = val<<1 | node.Val
    if node.Left == nil && node.Right == nil {
        return val
    }
    return dfs(node.Left, val) + dfs(node.Right, val)
}

func sumRootToLeaf(root *TreeNode) int {
    return dfs(root, 0)
}