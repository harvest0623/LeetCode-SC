class Solution:
    def maxLevelSum(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0       
        queue = deque([root])
        max_sum = float('-inf')
        result_level = 0
        current_level = 1        
        while queue:
            size = len(queue)
            level_sum = 0            
            for _ in range(size):
                node = queue.popleft()
                level_sum += node.val                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)            
            if level_sum > max_sum:
                max_sum = level_sum
                result_level = current_level           
            current_level += 1        
        return result_level