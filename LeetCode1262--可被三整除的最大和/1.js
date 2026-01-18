// 方法一：贪心
var maxSumDivThree = function(nums) {
    let total = 0;
    const mod1 = [], mod2 = [];
    
    for (const num of nums) {
        total += num;
        if (num % 3 === 1) mod1.push(num);
        else if (num % 3 === 2) mod2.push(num);
    }
    
    mod1.sort((a, b) => a - b);
    mod2.sort((a, b) => a - b);
    
    if (total % 3 === 0) {
        return total;
    } else if (total % 3 === 1) {
        let remove = Infinity;
        if (mod1.length >= 1) remove = Math.min(remove, mod1[0]);
        if (mod2.length >= 2) remove = Math.min(remove, mod2[0] + mod2[1]);
        
        return total - (remove === Infinity ? 0 : remove);
    } else { // total % 3 === 2
        let remove = Infinity;
        if (mod2.length >= 1) remove = Math.min(remove, mod2[0]);
        if (mod1.length >= 2) remove = Math.min(remove, mod1[0] + mod1[1]);
        
        return total - (remove === Infinity ? 0 : remove);
    }
};

// 方法二：动态规划
var maxSumDivThree = function(nums) {
    let dp = [0, -Infinity, -Infinity];
    for (let num of nums) {
        let new_dp = [...dp];  
        for (let i = 0; i < 3; i++) {
            if (dp[i] !== -Infinity) {
                let new_sum = dp[i] + num;
                let new_remainder = new_sum % 3;
                new_dp[new_remainder] = Math.max(new_dp[new_remainder], new_sum);
            }
        }
        dp = new_dp;
    }   
    return dp[0] > 0 ? dp[0] : 0;
};