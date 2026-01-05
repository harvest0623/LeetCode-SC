var permute = function(nums) {
    const n = nums.length;
    const path = Array(n).fill(0);
    const onPath = Array(n).fill(false); 
    const ans = [];
    function dfs(i) {
        if (i === n) {
            ans.push(path.slice());
            return;
        }
        for (let j = 0; j < n; j++) {
            if (!onPath[j]) {
                path[i] = nums[j]; 
                onPath[j] = true; 
                dfs(i + 1);
                onPath[j] = false; 
            }
        }
    };
    dfs(0);
    return ans;
};