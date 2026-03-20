public class Solution {
    const int MOD = 1000000007;
    int[][][] memo;
    int limit;

    public int NumberOfStableArrays(int zero, int one, int limit) {
        this.memo = new int[zero + 1][][];
        for (int i = 0; i <= zero; i++) {
            memo[i] = new int[one + 1][];
            for (int j = 0; j <= one; j++) {
                memo[i][j] = new int[2];
                Array.Fill(memo[i][j], -1);
            }
        }
        this.limit = limit;
        return (DP(zero, one, 0) + DP(zero, one, 1)) % MOD;
    }

    public int DP(int zero, int one, int lastBit) {
        if (zero == 0) {
            return (lastBit == 0 || one > limit) ? 0 : 1;
        } else if (one == 0) {
            return (lastBit == 1 || zero > limit) ? 0 : 1;
        }

        if (memo[zero][one][lastBit] == -1) {
            int res = 0;
            if (lastBit == 0) {
                res = (DP(zero - 1, one, 0) + DP(zero - 1, one, 1))% MOD;
                if (zero > limit) {
                    res = (res - DP(zero - limit - 1, one, 1) + MOD) % MOD;
                }
            } else {
                res = (DP(zero, one - 1, 0) + DP(zero, one - 1, 1)) % MOD;
                if (one > limit) {
                    res = (res - DP(zero, one - limit - 1, 0) + MOD) % MOD;
                }
            }
            memo[zero][one][lastBit] = res % MOD;
        }
        return memo[zero][one][lastBit];
    }
}