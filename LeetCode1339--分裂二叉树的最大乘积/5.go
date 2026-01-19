func maxProduct(root *TreeNode) int {
    sum := 0
    best := 0
    
    var dfs func(*TreeNode)
    dfs = func(node *TreeNode) {
        if node == nil {
            return
        }
        sum += node.Val
        dfs(node.Left)
        dfs(node.Right)
    }
    
    var dfs2 func(*TreeNode) int
    dfs2 = func(node *TreeNode) int {
        if node == nil {
            return 0
        }
        cur := dfs2(node.Left) + dfs2(node.Right) + node.Val
        if abs(cur*2 - sum) < abs(best*2 - sum) {
            best = cur
        }
        return cur
    }
    
    dfs(root)
    dfs2(root)
    return best * (sum - best) % 1000000007
}

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}