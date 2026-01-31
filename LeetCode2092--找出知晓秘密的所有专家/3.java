class Solution {
    public List<Integer> findAllPeople(int n, int[][] meetings, int firstPerson) {
        int m = meetings.length;
        Arrays.sort(meetings, (x, y) -> Integer.compare(x[2], y[2]));
        boolean[] secret = new boolean[n];
        secret[0] = true;
        secret[firstPerson] = true;
        
        for (int i = 0; i < m;) {
            // meetings[i .. j] 为同一时间
            int j = i;
            while (j + 1 < m && meetings[j + 1][2] == meetings[i][2]) {
                ++j;
            }
            
            Set<Integer> vertices = new HashSet<>();
            Map<Integer, List<Integer>> edges = new HashMap<>();
            for (int k = i; k <= j; ++k) {
                int x = meetings[k][0], y = meetings[k][1];
                vertices.add(x);
                vertices.add(y);
                edges.computeIfAbsent(x, key -> new ArrayList<>()).add(y);
                edges.computeIfAbsent(y, key -> new ArrayList<>()).add(x);
            }
            
            Queue<Integer> q = new LinkedList<>();
            for (int u : vertices) {
                if (secret[u]) {
                    q.offer(u);
                }
            }
            while (!q.isEmpty()) {
                int u = q.poll();
                if (edges.containsKey(u)) {
                    for (int v : edges.get(u)) {
                        if (!secret[v]) {
                            secret[v] = true;
                            q.offer(v);
                        }
                    }
                }
            }
            
            i = j + 1;
        }
        
        List<Integer> ans = new ArrayList<>();
        for (int i = 0; i < n; ++i) {
            if (secret[i]) {
                ans.add(i);
            }
        }
        return ans;
    }
}