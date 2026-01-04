var maxArea = function(height) {
    let ans = 0, left = 0, right = height.length - 1;
    while (left < right) {
        const area = (right - left) * Math.min(height[left], height[right]);
        ans = Math.max(ans, area);
        if (height[left] < height[right]) {
            // height[left] 与右边的任意垂线都无法组成一个比 ans 更大的面积
            left++;
        } else {
            // height[right] 与左边的任意垂线都无法组成一个比 ans 更大的面积
            right--;
        }
    }
    return ans;
};