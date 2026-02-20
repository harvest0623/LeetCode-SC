var minimumOperations = function(nums) {
    return _.sumBy(nums, x => (x % 3 !== 0 ? 1 : 0));
};