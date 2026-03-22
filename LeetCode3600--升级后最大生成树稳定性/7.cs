public class DSU {
    public int[] parent;

    public DSU(int[] parent) {
        this.parent = parent;
    }

    public int Find(int x) {
        return this.parent[x] == x ? x : (this.parent[x] = this.Find(this.parent[x]));
    }

    public void Join(int x, int y) {
        int px = this.Find(x);
        int py = this.Find(y);
        this.parent[px] = py;
    }
}

public class Solution {
    const int MAX_STABILITY = 200000;

    public int MaxStability(int n, int[][] edges, int k) {
        int ans = -1;

        if (edges.Length < n - 1) {
            return -1;
        }

        List<int[]> mustEdges = new List<int[]>();
        List<int[]> optionalEdges = new List<int[]>();

        foreach (var edge in edges) {
            if (edge[3] == 1) {
                mustEdges.Add(edge);
            } else {
                optionalEdges.Add(edge);
            }
        }

        if (mustEdges.Count > n - 1) {
            return -1;
        }
        optionalEdges.Sort((a, b) => b[2].CompareTo(a[2]));

        int selectedInit = 0;
        int mustMinStability = MAX_STABILITY;
        int[] initialParent = Enumerable.Range(0, n).ToArray();
        DSU dsuInit = new DSU(initialParent);

        foreach (var edge in mustEdges) {
            if (dsuInit.Find(edge[0]) == dsuInit.Find(edge[1]) || selectedInit == n - 1) {
                return -1;
            }

            dsuInit.Join(edge[0], edge[1]);
            selectedInit++;
            mustMinStability = Math.Min(mustMinStability, edge[2]);
        }

        int l = 0;
        int r = mustMinStability;

        while (l < r) {
            int mid = l + ((r - l + 1) >> 1);

            DSU dsu = new DSU((int[])dsuInit.parent.Clone());

            int selected = selectedInit;
            int doubledCount = 0;

            foreach (var edge in optionalEdges) {
                if (dsu.Find(edge[0]) == dsu.Find(edge[1])) {
                    continue;
                }

                if (edge[2] >= mid) {
                    dsu.Join(edge[0], edge[1]);
                    selected++;
                } else if (doubledCount < k && edge[2] * 2 >= mid) {
                    doubledCount++;
                    dsu.Join(edge[0], edge[1]);
                    selected++;
                } else {
                    break;
                }

                if (selected == n - 1) {
                    break;
                }
            }

            if (selected != n - 1) {
                r = mid - 1;
            } else {
                ans = l = mid;
            }
        }

        return ans;
    }
}