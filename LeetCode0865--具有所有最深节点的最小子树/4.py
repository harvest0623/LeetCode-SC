class Solution:
    def subtreeWithAllDeepest(self, root: TreeNode) -> TreeNode:
        def dfs(root):
            if not root:
                return 0, None
            d1, lca1 = dfs(root.left)
            d2, lca2 = dfs(root.right)
            if d1 > d2:
                return d1 + 1, lca1
            if d1 < d2:
                return d2 + 1, lca2
            return d1 + 1, root
            
        return dfs(root)[1]