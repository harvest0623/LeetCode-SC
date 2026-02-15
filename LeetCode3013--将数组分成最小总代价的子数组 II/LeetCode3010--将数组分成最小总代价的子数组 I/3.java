class Solution {
    public long minimumCost(int[] nums, int k, int dist) {
        k--;
        sumL = nums[0];
        for (int i = 1; i < dist + 2; i++) {
            sumL += nums[i];
            L.merge(nums[i], 1, Integer::sum);
        }
        sizeL = dist + 1;
        while (sizeL > k) {
            l2r();
        }

        long ans = sumL;
        for (int i = dist + 2; i < nums.length; i++) {
            // 移除 out
            int out = nums[i - dist - 1];
            if (L.containsKey(out)) {
                sumL -= out;
                sizeL--;
                removeOne(L, out);
            } else {
                removeOne(R, out);
            }

            // 添加 in
            int in = nums[i];
            if (in < L.lastKey()) {
                sumL += in;
                sizeL++;
                L.merge(in, 1, Integer::sum);
            } else {
                R.merge(in, 1, Integer::sum);
            }

            // 维护大小
            if (sizeL == k - 1) {
                r2l();
            } else if (sizeL == k + 1) {
                l2r();
            }

            ans = Math.min(ans, sumL);
        }
        return ans;
    }

    private long sumL;
    private int sizeL;
    private final TreeMap<Integer, Integer> L = new TreeMap<>();
    private final TreeMap<Integer, Integer> R = new TreeMap<>();

    private void l2r() {
        int x = L.lastKey();
        removeOne(L, x);
        sumL -= x;
        sizeL--;
        R.merge(x, 1, Integer::sum);
    }

    private void r2l() {
        int x = R.firstKey();
        removeOne(R, x);
        sumL += x;
        sizeL++;
        L.merge(x, 1, Integer::sum);
    }

    private void removeOne(Map<Integer, Integer> m, int x) {
        int cnt = m.get(x);
        if (cnt > 1) {
            m.put(x, cnt - 1);
        } else {
            m.remove(x);
        }
    }
}