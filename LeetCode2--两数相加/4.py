class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        if l1.val+l2.val >= 10:
            i = 1
        else:
            i = 0
        node1 = ListNode((l1.val+l2.val)%10)
        node = node1
        if l1.next!=None and l2.next!=None:
            while l1.next and l2.next:
                node.next = ListNode((l1.next.val+l2.next.val+i)%10)
                if l1.next.val+l2.next.val+i >= 10:
                    i = 1
                else:
                    i = 0
                l1 = l1.next
                l2 = l2.next
                node = node.next
        if l1.next != None:
            while l1.next:
                node.next = ListNode((l1.next.val+i)%10)
                if l1.next.val+i >= 10:
                    i = 1
                else:
                    i = 0
                l1 = l1.next
                node = node.next
        if l2.next != None:
            while l2.next:
                node.next = ListNode((l2.next.val+i)%10)
                if l2.next.val+i >= 10:
                    i = 1
                else:
                    i = 0
                l2 = l2.next
                node = node.next
        if i == 1:
            node.next = ListNode(1)
        return node1