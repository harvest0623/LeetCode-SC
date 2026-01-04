// 方法一: 递归
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None, // 如果两个链表都为空，返回空
            (Some(node1), None) => Some(node1), // 如果第二个链表为空，返回第一个链表
            (None, Some(node2)) => Some(node2), // 如果第一个链表为空，返回第二个链表
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge_two_lists(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_two_lists(Some(node1), node2.next);
                    Some(node2)
                }
            }
        }
    }
}

// 方法二: 迭代
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0); 
        let mut cur = &mut dummy; 
        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur.next = list1.take(); 
                cur = cur.next.as_mut()?;
                list1 = cur.next.take();
            } else { 
                cur.next = list2.take();
                cur = cur.next.as_mut()?;
                list2 = cur.next.take();
            };
        }
        cur.next = list1.or(list2); 
        dummy.next
    }
}