class Solution {
    public int swimInWater(int[][] grid) {
        int n = grid.length;
        PriorityQueue<Entry> pq = new PriorityQueue<Entry>(new Comparator<Entry>() {
            public int compare(Entry entry1, Entry entry2) {
                return entry1.val - entry2.val;
            }
        });
        boolean[][] visited = new boolean[n][n];

        pq.offer(new Entry(0, 0, grid[0][0]));
        int ret = 0;
        int[][] directions = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
        while (!pq.isEmpty()) {
            Entry x = pq.poll();
            if (visited[x.i][x.j]) {
                continue;
            }
            
            visited[x.i][x.j] = true;
            ret = Math.max(ret, grid[x.i][x.j]);
            if (x.i == n - 1 && x.j == n - 1) {
                break;
            }

            for (int[] direction : directions) {
                int ni = x.i + direction[0], nj = x.j + direction[1];
                if (ni >= 0 && ni < n && nj >= 0 && nj < n) {
                    if (!visited[ni][nj]) {
                        pq.offer(new Entry(ni, nj, grid[ni][nj]));
                    }
                }
            }
        }
        return ret;
    }
}

class Entry {
    int i;
    int j;
    int val;
    public Entry(int i, int j, int val) {
        this.i = i;
        this.j = j;
        this.val = val;
    }
};