class Solution {
    public int maximizeSquareHoleArea(int n, int m, int[] hBars, int[] vBars) {
        int side = Math.min(f(hBars), f(vBars)) + 1;
        return side * side;
    }

    // 返回 a 排序后的最长连续递增子数组的长度
    private int f(int[] a) {
        Arrays.sort(a);
        int mx = 1;
        int cnt = 1;
        for (int i = 1; i < a.length; i++) {
            if (a[i] == a[i - 1] + 1) {
                cnt++;
                mx = Math.max(mx, cnt);
            } else {
                cnt = 1; // 重新计数
            }
        }
        return mx;
    }
}