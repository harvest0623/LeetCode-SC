class Solution:
    def minDeletionSize(self, A):
        cuts = [False] * (len(A) - 1)
        ans = 0
        for col in zip(*A):
            if all(cuts[i] or col[i] <= col[i + 1] for i in range(len(col) - 1)):
                for i in range(len(col) - 1):
                    if col[i] < col[i + 1]:
                        cuts[i] = True
            else:
                ans += 1
        return ans