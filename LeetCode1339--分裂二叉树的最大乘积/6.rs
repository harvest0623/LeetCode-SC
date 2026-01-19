use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut best = 0;
        
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(n) = node {
                let node_ref = n.borrow();
                *sum += node_ref.val;
                dfs(&node_ref.left, sum);
                dfs(&node_ref.right, sum);
            }
        }
        
        fn dfs2(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32, best: &mut i32) -> i32 {
            if let Some(n) = node {
                let node_ref = n.borrow();
                let cur = dfs2(&node_ref.left, sum, best) + 
                         dfs2(&node_ref.right, sum, best) + 
                         node_ref.val;
                if (cur * 2 - sum).abs() < (*best * 2 - sum).abs() {
                    *best = cur;
                }
                cur
            } else {
                0
            }
        }
        
        dfs(&root, &mut sum);
        dfs2(&root, sum, &mut best);
        ((best as i64) * ((sum - best) as i64) % 1000000007) as i32
    }
}