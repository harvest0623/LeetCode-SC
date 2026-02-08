use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn get_height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let left_h = get_height(&node.left);
                let right_h = get_height(&node.right);
                if left_h == -1 || right_h == -1 || (left_h - right_h).abs() > 1 {
                    return -1;
                }
                return left_h.max(right_h) + 1;
            }
            0
        }
        get_height(&root) != -1
    }
}