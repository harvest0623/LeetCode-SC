class Solution {
    public int maxFrequencyElements(int[] nums) {
        Map<Integer, Integer> cnt = new HashMap<>(); // 更快的写法见【Java 数组】
        int maxCnt = 0;
        int ans = 0;
        for (int x : nums) {
            int c = cnt.merge(x, 1, Integer::sum); // c = ++cnt[x]
            if (c > maxCnt) {
                ans = maxCnt = c;
            } else if (c == maxCnt) {
                ans += c;
            }
        }
        return ans;
    }
}

// 数组
class Solution {
    public int maxFrequencyElements(int[] nums) {
        int mx = 0;
        for (int x : nums) {
            mx = Math.max(mx, x);
        }
        
        int[] cnt = new int[mx + 1];
        int maxCnt = 0;
        int ans = 0;
        for (int x : nums) {
            int c = ++cnt[x];
            if (c > maxCnt) {
                ans = maxCnt = c;
            } else if (c == maxCnt) {
                ans += c;
            }
        }
        return ans;
    }
}