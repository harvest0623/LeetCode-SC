class Solution {
    public int maxTwoEvents(int[][] events) {
        // 按照结束时间排序
        Arrays.sort(events, (a, b) -> a[1] - b[1]);

        // 从栈底到栈顶，结束时间递增，价值递增
        ArrayList<int[]> st = new ArrayList<>(); // (结束时间, 价值)
        st.add(new int[]{0, 0}); // 栈底哨兵

        int ans = 0;
        for (int[] e : events) {
            int i = search(st, e[0]);
            int value = e[2];
            ans = Math.max(ans, st.get(i)[1] + value);
            // 遇到比栈顶更大的价值，入栈
            if (value > st.getLast()[1]) {
                st.add(new int[]{e[1], value});
            }
        }
        return ans;
    }

    // 返回最后一个满足 st[i][0] < target 的 i
    private int search(List<int[]> st, int target) {
        int left = -1, right = st.size();
        while (left + 1 < right) { // 开区间二分
            int mid = left + (right - left) / 2;
            if (st.get(mid)[0] < target) {
                left = mid;
            } else {
                right = mid;
            }
        }
        return left;
    }
}