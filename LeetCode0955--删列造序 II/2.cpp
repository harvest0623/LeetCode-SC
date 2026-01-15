class Solution {
public:
    int minDeletionSize(vector<string>& A) {
        vector<bool> cuts(A.size() - 1, false);
        int ans = 0;
        for (int j = 0; j < A[0].size(); j++) {
            bool canKeep = true;
            for (int i = 0; i < A.size() - 1; i++) {
                if (!cuts[i] && A[i][j] > A[i + 1][j]) {
                    canKeep = false;
                    break;
                }
            }
            if (canKeep) {
                for (int i = 0; i < A.size() - 1; i++) {
                    if (A[i][j] < A[i + 1][j]) {
                        cuts[i] = true;
                    }
                }
            } else {
                ans++;
            }
        }
        return ans;
    }
}; 