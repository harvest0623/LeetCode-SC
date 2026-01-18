class Solution {
    public int minTimeToVisitAllPoints(int[][] points) {
        int total = 0;
        int n = points.length;
        for (int i = 1; i < n; i++) {
            int dx = Math.abs(points[i][0] - points[i-1][0]);
            int dy = Math.abs(points[i][1] - points[i-1][1]);
            total += Math.max(dx, dy);
        }
        return total;
    }
}