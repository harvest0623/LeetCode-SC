type TrieNode struct {
    son [26]*TrieNode
    sid int
}
func minimumCost(source string, target string, original []string, changed []string, cost []int) int64 {
    root := &TrieNode{sid: -1}
    sid := 0    
    getId := func(s string) int {
        node := root
        for i := 0; i < len(s); i++ {
            idx := s[i] - 'a'
            if node.son[idx] == nil {
                node.son[idx] = &TrieNode{sid: -1}
            }
            node = node.son[idx]
        }
        if node.sid < 0 {
            node.sid = sid
            sid++
        }
        return node.sid
    }    
    // 初始化距离矩阵
    m := len(cost)
    INF := int(1e18)
    dis := make([][]int, m*2)
    for i := range dis {
        dis[i] = make([]int, m*2)
        for j := range dis[i] {
            dis[i][j] = INF
        }
        dis[i][i] = 0
    }  
    for i := 0; i < m; i++ {
        x := getId(original[i])
        y := getId(changed[i])
        if cost[i] < dis[x][y] {
            dis[x][y] = cost[i]
        }
    }    
    // Floyd 算法求任意两点最短路
    for k := 0; k < sid; k++ {
        for i := 0; i < sid; i++ {
            if dis[i][k] == INF {
                continue
            }
            for j := 0; j < sid; j++ {
                if dis[i][k]+dis[k][j] < dis[i][j] {
                    dis[i][j] = dis[i][k] + dis[k][j]
                }
            }
        }
    }   
    n := len(source)
    f := make([]int64, n+1)
    INF64 := int64(1e18)  
    for i := n - 1; i >= 0; i-- {
        // 不修改 source[i]
        if source[i] == target[i] {
            f[i] = f[i+1]
        } else {
            f[i] = INF64
        }       
        p := root
        q := root        
        for j := i; j < n; j++ {
            pIdx := source[j] - 'a'
            qIdx := target[j] - 'a'            
            if p.son[pIdx] == nil || q.son[qIdx] == nil {
                break
            }           
            p = p.son[pIdx]
            q = q.son[qIdx]        
            if p.sid < 0 || q.sid < 0 {
                continue
            }        
            // 修改从 i 到 j 的这一段
            d := dis[p.sid][q.sid]
            if d < INF {
                newCost := int64(d) + f[j+1]
                if newCost < f[i] {
                    f[i] = newCost
                }
            }
        }
    }  
    if f[0] < INF64 {
        return f[0]
    }
    return -1
}