var maxLevelSum = function(root) {
    if (!root) return 0;
    const queue = [root];
    let maxSum = -Infinity;
    let resultLevel = 0;
    let currentLevel = 1;
    while (queue.length > 0) {
        const size = queue.length;
        let levelSum = 0;
        for (let i = 0; i < size; i++) {
            const node = queue.shift();
            levelSum += node.val;
            if (node.left) queue.push(node.left);
            if (node.right) queue.push(node.right);
        }
        if (levelSum > maxSum) {
            maxSum = levelSum;
            resultLevel = currentLevel;
        }
        currentLevel++;
    }
    return resultLevel;
};