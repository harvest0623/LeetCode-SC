var balanceBST = function(root) {
    const inorderSeq = [];
    
    const getInorder = (o) => {
        if (o.left) {
            getInorder(o.left);
        }
        inorderSeq.push(o.val);
        if (o.right) {
            getInorder(o.right);
        }
    };
    
    const build = (l, r) => {
        const mid = (l + r) >> 1;
        const o = new TreeNode(inorderSeq[mid]);
        if (l <= mid - 1) {
            o.left = build(l, mid - 1);
        }
        if (mid + 1 <= r) {
            o.right = build(mid + 1, r);
        }
        return o;
    };
    
    getInorder(root);
    return build(0, inorderSeq.length - 1);
};