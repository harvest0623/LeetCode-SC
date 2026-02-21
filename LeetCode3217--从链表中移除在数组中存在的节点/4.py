class Solution:
    def modifiedList(self, nums: List[int], head: Optional[ListNode]) -> Optional[ListNode]:
        num_set = set(nums)
        sentry = ListNode(0, head)
        p = sentry
        while p.next is not None:
            if p.next.val in num_set:
                p.next = p.next.next
            else:
                p = p.next
        return sentry.next