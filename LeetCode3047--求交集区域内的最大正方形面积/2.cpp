class Solution {
public:
    long long largestSquareArea(vector<vector<int>>& bottomLeft,
                                vector<vector<int>>& topRight) {
        int n = bottomLeft.size();
        long long maxArea = 0;
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                int x1 = max(bottomLeft[i][0], bottomLeft[j][0]);
                int x2 = min(topRight[i][0], topRight[j][0]);
                int y1 = max(bottomLeft[i][1], bottomLeft[j][1]);
                int y2 = min(topRight[i][1], topRight[j][1]);
                if (x1 < x2 && y1 < y2) {
                    long long side = min(x2 - x1, y2 - y1);
                    maxArea = max(maxArea, side * side);
                }
            }
        }
        return maxArea;
    }
};