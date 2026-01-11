class Solution {
    public boolean pyramidTransition(String bottom, List<String> allowed) {
        List<Character>[][] groups = new ArrayList[6][6];
        for (List<Character>[] row : groups) {
            Arrays.setAll(row, _ -> new ArrayList<>());
        }
        for (String S : allowed) {
            char[] s = S.toCharArray();
            groups[s[0] - 'A'][s[1] - 'A'].add(s[2]);
        }
        int n = bottom.length();
        char[][] pyramid = new char[n][];
        for (int i = 0; i < n - 1; i++) {
            pyramid[i] = new char[i + 1];
        }
        pyramid[n - 1] = bottom.toCharArray();
        Set<String> vis = new HashSet<>();
        return dfs(n - 2, 0, pyramid, vis, groups);
    }
    private boolean dfs(int i, int j, char[][] pyramid, Set<String> vis, List<Character>[][] groups) {
        if (i < 0) {
            return true;
        }
        if (j == i + 1) {
            String row = new String(pyramid[i]);
            if (!vis.add(row)) { 
                return false; 
            }
            return dfs(i - 1, 0, pyramid, vis, groups);
        }
        for (char top : groups[pyramid[i + 1][j] - 'A'][pyramid[i + 1][j + 1] - 'A']) {
            pyramid[i][j] = top;
            if (dfs(i, j + 1, pyramid, vis, groups)) {
                return true;
            }
        }
        return false;
    }
}