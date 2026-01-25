impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min_diff = i32::MAX;
        let mut res = Vec::new();
        // 找出最小差值
        for i in 1..arr.len() {
            min_diff = min_diff.min(arr[i] - arr[i - 1]);
        }
        // 收集结果
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == min_diff {
                res.push(vec![arr[i - 1], arr[i]]);
            }
        }
        res
    }
}