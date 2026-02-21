var modifiedList = function(nums, head) {
    const isExist = new Set(nums);
    const sentry = new ListNode(0, head);
    let p = sentry;
    while (p.next) {
        if (isExist.has(p.next.val)) {
            p.next = p.next.next;
        } else {
            p = p.next;
        }
    }
    return sentry.next;
};