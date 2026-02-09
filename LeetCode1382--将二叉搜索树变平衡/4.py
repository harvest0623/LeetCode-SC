class Solution:
    def balanceBST(self, root: TreeNode) -> TreeNode:
        def getInorder(o):
            if o.left:
                getInorder(o.left)
            inorderSeq.append(o.val)
            if o.right:
                getInorder(o.right)
        
        def build(l, r):
            mid = (l + r) // 2
            o = TreeNode(inorderSeq[mid])
            if l <= mid - 1:
                o.left = build(l, mid - 1)
            if mid + 1 <= r:
                o.right = build(mid + 1, r)
            return o

        inorderSeq = list()
        getInorder(root)
        return build(0, len(inorderSeq) - 1)