use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_seq = Vec::new();
        
        Self::get_inorder(&root, &mut inorder_seq);
        Self::build(&inorder_seq, 0, inorder_seq.len() as i32 - 1)
    }
    
    fn get_inorder(root: &Option<Rc<RefCell<TreeNode>>>, seq: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            Self::get_inorder(&node_ref.left, seq);
            seq.push(node_ref.val);
            Self::get_inorder(&node_ref.right, seq);
        }
    }
    
    fn build(seq: &[i32], l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }
        let mid = (l + r) >> 1;
        let mut node = TreeNode::new(seq[mid as usize]);
        node.left = Self::build(seq, l, mid - 1);
        node.right = Self::build(seq, mid + 1, r);
        Some(Rc::new(RefCell::new(node)))
    }
}