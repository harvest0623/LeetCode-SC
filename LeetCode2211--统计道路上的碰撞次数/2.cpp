class Solution {
public:
    int countCollisions(string s) { 
        int n = s.size();
        // 前缀向左的车不会发生碰撞
        int l = 0;
        while (l < n && s[l] == 'L') {
            l++;
        }
        // 后缀向右的车不会发生碰撞
        int r = n; 
        while (r > l && s[r - 1] == 'R') {
            r--;
        }
        // 剩下非静止的车计入碰撞次数
        int cnt = 0; 
        for (int i = l; i < r; i++) {
            cnt += s[i] != 'S';
        }
        return cnt;
    }
};