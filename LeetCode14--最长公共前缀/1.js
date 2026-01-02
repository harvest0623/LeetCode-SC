var longestCommonPrefix = function(strs) {
    const s0 = strs[0];
    for (let j = 0; j < s0.length; j++) { 
        for (const s of strs) { 
            if (j === s.length || s[j] !== s0[j]) { 
                return s0.slice(0, j); 
            }
        }
    }
    return s0;
};