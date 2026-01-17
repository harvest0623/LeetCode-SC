var prefixesDivBy5 = function(nums) {
    const answer = [];
    let prefix = 0;
    const length = nums.length;
    for (let i = 0; i < length; i++) {
        prefix = ((prefix << 1) + nums[i]) % 5;
        answer.push(prefix === 0);
    }
    return answer;
};