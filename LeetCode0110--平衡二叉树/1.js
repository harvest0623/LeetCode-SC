function getHeight(node) {
    if (node === null) {
        return 0;
    }
    const leftH = getHeight(node.left);
    const rightH = getHeight(node.right);
    if (leftH === -1 || rightH === -1 || Math.abs(leftH - rightH) > 1) {
        return -1;
    }
    return Math.max(leftH, rightH) + 1;
}

var isBalanced = function(root) {
    return getHeight(root) !== -1;
};