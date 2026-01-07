class Solution {
public:
    string fractionToDecimal(int numerator, int denominator) {
        long long a = numerator, b = denominator;
        string sign = a * b < 0 ? "-" : "";
        a = abs(a); 
        b = abs(b);
        long long q = a / b, r = a % b;
        if (r == 0) { 
            return sign + to_string(q);
        }
        string ans = sign + to_string(q) + ".";
        unordered_map<long long, int> r_to_pos = {{r, ans.size()}}; // 记录初始余数对应位置
        while (r) {
            r *= 10;
            q = r / b;
            r %= b;
            ans += '0' + q;
            if (r_to_pos.contains(r)) { 
                int pos = r_to_pos[r];
                return ans.substr(0, pos) + "(" + ans.substr(pos) + ")";
            }
            r_to_pos[r] = ans.size();
        }
        return ans; 
    }
};