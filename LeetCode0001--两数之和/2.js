var twoSum = function (arr, res) {
    var diffs = {};
    var len = arr.length;
    for (var i = 0; i < len; i++) {
        if (diffs[res - arr[i]] !== undefined) {
            return [diffs[res - arr[i]], i];
        }
        diffs[arr[i]] = i;
    }
};