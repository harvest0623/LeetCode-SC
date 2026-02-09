func balanceBST(root *TreeNode) *TreeNode {
    var inorderSeq []int
    
    var getInorder func(*TreeNode)
    getInorder = func(o *TreeNode) {
        if o.Left != nil {
            getInorder(o.Left)
        }
        inorderSeq = append(inorderSeq, o.Val)
        if o.Right != nil {
            getInorder(o.Right)
        }
    }
    
    var build func(int, int) *TreeNode
    build = func(l, r int) *TreeNode {
        mid := (l + r) >> 1
        o := &TreeNode{Val: inorderSeq[mid]}
        if l <= mid-1 {
            o.Left = build(l, mid-1)
        }
        if mid+1 <= r {
            o.Right = build(mid+1, r)
        }
        return o
    }
    
    getInorder(root)
    return build(0, len(inorderSeq)-1)
}