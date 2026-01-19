class Solution:
    def maxProduct(self, root: Optional[TreeNode]) -> int:
        self.sum = 0
        self.best = 0
        
        def dfs(node):
            if not node:
                return
            self.sum += node.val
            dfs(node.left)
            dfs(node.right)
        
        def dfs2(node):
            if not node:
                return 0
            cur = dfs2(node.left) + dfs2(node.right) + node.val
            if abs(cur * 2 - self.sum) < abs(self.best * 2 - self.sum):
                self.best = cur
            return cur
        
        dfs(root)
        dfs2(root)
        return self.best * (self.sum - self.best) % 1000000007