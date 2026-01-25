// 方法一：BFS 搜索 + 转轮
class Solution {
    public String findLexSmallestString(String s, int a, int b) {
        int n = s.length();
        Set<String> visited = new HashSet<>();
        Queue<String> q = new LinkedList<>();
        q.offer(s);
        visited.add(s);
        String ans = s;
        
        while (!q.isEmpty()) {
            String cur = q.poll();
            if (cur.compareTo(ans) < 0) ans = cur;
            
            // 操作1
            char[] arr = cur.toCharArray();
            for (int i = 1; i < n; i += 2) {
                int digit = arr[i] - '0';
                digit = (digit + a) % 10;
                arr[i] = (char) (digit + '0');
            }
            String op1 = new String(arr);
            if (!visited.contains(op1)) {
                visited.add(op1);
                q.offer(op1);
            }
            
            // 操作2
            String op2 = cur.substring(n - b) + cur.substring(0, n - b);
            if (!visited.contains(op2)) {
                visited.add(op2);
                q.offer(op2);
            }
        }
        return ans;
    }
}

// 方法二：数学 + 法转轮
class Solution {
    private int gcd(int a, int b) {
        return b == 0 ? a : gcd(b, a % b);
    }
    public String findLexSmallestString(String s, int a, int b) {
        int n = s.length();
        int g = gcd(a, 10);
        int step = gcd(n, b);
        String ans = s;
        for (int i = 0; i < n; i += step) {
            // 轮转
            String t = s.substring(i) + s.substring(0, i);
            char[] arr = t.toCharArray();
            // 调整函数
            change(arr, g, 1, n);  // 调整奇数位
            if ((step & 1) == 1) {
                change(arr, g, 0, n);  // 调整偶数位
            }
            String candidate = new String(arr);
            if (candidate.compareTo(ans) < 0) {
                ans = candidate;
            }
        }
        return ans;
    }
    
    private void change(char[] arr, int g, int start, int n) {
        int ch = arr[start] - '0';
        int inc = (ch % g + 10 - ch) % 10;
        for (int j = start; j < n; j += 2) {
            int digit = (arr[j] - '0' + inc) % 10;
            arr[j] = (char)(digit + '0');
        }
    }
}