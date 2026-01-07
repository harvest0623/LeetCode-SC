impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut cur = head.as_mut()?;
        while let Some(nxt) = cur.next.take() { 
            if (nxt.val == cur.val) {
                cur.next = nxt.next; 
            } else {
                cur.next = Some(nxt);
                cur = cur.next.as_mut()?;
            }
        }
        head
    }
}