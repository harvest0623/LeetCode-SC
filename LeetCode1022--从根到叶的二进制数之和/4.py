class Solution:
    def sumRootToLeaf(self, root: Optional[TreeNode]) -> int:
        def dfs(node: Optional[TreeNode], val: int) -> int:
            if node is None:
                return 0
            val = (val << 1) | node.val
            if node.left is None and node.right is None:
                return val
            return dfs(node.left, val) + dfs(node.right, val)
        return dfs(root, 0)