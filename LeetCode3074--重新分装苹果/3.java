class Solution {
    public int minimumBoxes(int[] apple, int[] capacity) {
        int s = 0;
        for (int x : apple) {
            s += x; 
        }
        Arrays.sort(capacity);
        int m = capacity.length;
        int i = m - 1; 
        while (s > 0) { 
            s -= capacity[i--]; 
        }
        return m - 1 - i;
    }
}