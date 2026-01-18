class Solution {
public:
    int maxLevelSum(TreeNode* root) {
        if (!root) return 0;
        queue<TreeNode*> q;
        q.push(root);        
        int maxSum = INT_MIN;
        int resultLevel = 0;
        int currentLevel = 1;        
        while (!q.empty()) {
            int size = q.size();
            int levelSum = 0;            
            for (int i = 0; i < size; i++) {
                TreeNode* node = q.front();
                q.pop();
                levelSum += node->val;                
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }            
            if (levelSum > maxSum) {
                maxSum = levelSum;
                resultLevel = currentLevel;
            }           
            currentLevel++;
        }       
        return resultLevel;
    }
};