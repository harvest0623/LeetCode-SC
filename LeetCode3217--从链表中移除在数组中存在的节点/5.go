func modifiedList(nums []int, head *ListNode) *ListNode {
    isExist := make(map[int]bool)
    for _, num := range nums {
        isExist[num] = true
    }
    sentry := &ListNode{Next: head}
    p := sentry
    for p.Next != nil {
        if isExist[p.Next.Val] {
            p.Next = p.Next.Next
        } else {
            p = p.Next
        }
    }
    return sentry.Next
}