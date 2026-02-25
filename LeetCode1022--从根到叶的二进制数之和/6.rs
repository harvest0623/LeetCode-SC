use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut val: i32) -> i32 {
            return if let Some(node) = root {
                val = (val << 1) | node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() { val } else { dfs(&node.borrow().left, val) + dfs(&node.borrow().right, val) }
            } else { 0 };
        }
        dfs(&root, 0)
    }
}