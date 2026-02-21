use std::collections::HashSet;
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let is_exist: HashSet<i32> = nums.into_iter().collect();
        let mut sentry = Box::new(ListNode::new(0));
        sentry.next = head;
        let mut p = &mut sentry;
        while let Some(ref mut next_node) = p.next {
            if is_exist.contains(&next_node.val) {
                p.next = next_node.next.take();
            } else {
                p = p.next.as_mut().unwrap();
            }
        }
        sentry.next
    }
}