impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }     
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());        
        let mut max_sum = i32::MIN;
        let mut result_level = 0;
        let mut current_level = 1;        
        while !queue.is_empty() {
            let size = queue.len();
            let mut level_sum = 0;            
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                level_sum += node_ref.val;                
                if let Some(left) = &node_ref.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(right) = &node_ref.right {
                    queue.push_back(Rc::clone(right));
                }
            }            
            if level_sum > max_sum {
                max_sum = level_sum;
                result_level = current_level;
            }            
            current_level += 1;
        }        
        result_level
    }
}