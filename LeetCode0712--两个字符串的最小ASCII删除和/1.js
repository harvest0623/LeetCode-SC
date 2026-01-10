var minimumDeleteSum = function(s1, s2) {
    let dp1 = new Array(s1.length + 1); dp1[0] = 0;
    for(let i = 1; i <= s1.length; i++)
        dp1[i] = dp1[i - 1] + s1.charCodeAt(i - 1);
    for(let j = 1; j <= s2.length; j++){
        let dp2 = new Array(s1.length + 1);
        dp2[0] = dp1[0] + s2.charCodeAt(j - 1);
        for(let i = 1; i <= s1.length; i++){
            let cost1 = dp2[i - 1] + s1.charCodeAt(i - 1);
            let cost2 = dp1[i] + s2.charCodeAt(j - 1);
            dp2[i] = Math.min(cost1, cost2);
            if(s1[i - 1] == s2[j - 1])
                dp2[i] = Math.min(dp2[i], dp1[i - 1]);
        }
        dp1 = dp2;
    }
    return dp1.at(-1);
};