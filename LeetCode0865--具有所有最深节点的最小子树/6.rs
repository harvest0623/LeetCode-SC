use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).1
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(node_ref) => {
                let node_borrow = node_ref.borrow();
                let (left_depth, left_lca) = Self::dfs(&node_borrow.left);
                let (right_depth, right_lca) = Self::dfs(&node_borrow.right);
                if left_depth > right_depth {
                    (left_depth + 1, left_lca)
                } else if left_depth < right_depth {
                    (right_depth + 1, right_lca)
                } else {
                    (left_depth + 1, Some(node_ref.clone()))
                }
            }
        }
    }
}