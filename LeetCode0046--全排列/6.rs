impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(i: usize, nums: &[i32], ans: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, on_path: &mut [bool]) {
            if i == nums.len() {
                ans.push(path.clone());
                return;
            }
            for j in 0..nums.len() {
                if !on_path[j] {
                    path[i] = nums[j];
                    on_path[j] = true; 
                    dfs(i + 1, nums, ans, path, on_path);
                    on_path[j] = false; 
                }
            }
        }
        let n = nums.len();
        let mut path = vec![0; n];
        let mut on_path = vec![false; n]; 
        let mut ans = vec![];
        dfs(0, &nums, &mut ans, &mut path, &mut on_path);
        ans
    }
}