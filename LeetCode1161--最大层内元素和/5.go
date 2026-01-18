func maxLevelSum(root *TreeNode) int {
    if root == nil {
        return 0
    }  
    queue := []*TreeNode{root}
    maxSum := -1 << 31 // 相当于 int 的最小值
    resultLevel := 0
    currentLevel := 1    
    for len(queue) > 0 {
        size := len(queue)
        levelSum := 0       
        for i := 0; i < size; i++ {
            node := queue[0]
            queue = queue[1:]
            levelSum += node.Val            
            if node.Left != nil {
                queue = append(queue, node.Left)
            }
            if node.Right != nil {
                queue = append(queue, node.Right)
            }
        }       
        if levelSum > maxSum {
            maxSum = levelSum
            resultLevel = currentLevel
        }        
        currentLevel++
    }    
    return resultLevel
}