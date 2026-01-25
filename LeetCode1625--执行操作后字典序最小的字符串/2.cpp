// 方法一：BFS 搜索 + 转轮
class Solution {
public:
    string findLexSmallestString(string s, int a, int b) {
        int n = s.size();
        unordered_set<string> visited;
        queue<string> q;
        string ans = s;
        q.push(s);
        visited.insert(s);
        
        while (!q.empty()) {
            string cur = q.front(); q.pop();
            if (cur < ans) ans = cur;
            
            // 操作1：奇数位加a
            string op1 = cur;
            for (int i = 1; i < n; i += 2) {
                op1[i] = ((op1[i] - '0' + a) % 10) + '0';
            }
            if (!visited.count(op1)) {
                visited.insert(op1);
                q.push(op1);
            }
            
            // 操作2：轮转b位
            string op2 = cur.substr(n - b) + cur.substr(0, n - b);
            if (!visited.count(op2)) {
                visited.insert(op2);
                q.push(op2);
            }
        }
        return ans;
    }
};

// 方法二：数学 + 法转轮
class Solution {
public:
    string findLexSmallestString(string s, int a, int b) {
        int n = s.size();
        int g = gcd(a, 10);
        int step = gcd(n ,b);
        string ans = s;
        for(int i = 0; i < n; i += step){
            // 轮转
            string t = s.substr(i) + s.substr(0, i);
            auto change = [&](int start) -> void{
                int ch = t[start] - '0';
                int inc = (ch % g + 10 - ch) % 10;
                for(int j = start; j < n; j += 2){
                    t[j] = (t[j] - '0' + inc) % 10 + '0';
                }
            };
            change(1); // 调整奇数位
            if(step & 1){
                change(0); // 调整偶数位
            }
            if(ans.empty() || t < ans){
                ans = move(t);
            }
        }
        return ans;
    }
};