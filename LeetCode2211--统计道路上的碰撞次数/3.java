class Solution {
    public int countCollisions(String s) {
        int n = s.length();
        // 前缀向左的车不会发生碰撞
        int l = 0;
        while (l < n && s.charAt(l) == 'L') {
            l++;
        }
        // 后缀向右的车不会发生碰撞
        int r = n; 
        while (r > l && s.charAt(r - 1) == 'R') {
            r--;
        }
        // 剩下非静止的车计入碰撞次数
        int cnt = 0; 
        for (int i = l; i < r; i++) {
            if (s.charAt(i) != 'S') {
                cnt++;
            }
        }
        return cnt;
    }
}