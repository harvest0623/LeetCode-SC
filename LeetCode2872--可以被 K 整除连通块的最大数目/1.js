var maxKDivisibleComponents = function(n, edges, values, k) {
    const graph = Array.from({ length: n }, () => []);
    for (const edge of edges) {
        const u = edge[0], v = edge[1];
        graph[u].push(v);
        graph[v].push(u);
    }
    let result = 0;
    function dfs(node, parent) {
        let sum = values[node];
        for (const neighbor of graph[node]) {
            if (neighbor !== parent) {
                sum += dfs(neighbor, node);
            }
        }
        if (sum % k === 0) {
            result++;
        }
        return sum;
    }
    dfs(0, -1);
    return result;
};