class Solution {
public:
    string addBinary(string a, string b) {
        string ans;
        int i = a.size() - 1; 
        int j = b.size() - 1;
        int carry = 0; 
        while (i >= 0 || j >= 0 || carry) {
            int x = i >= 0 ? a[i] - '0' : 0;
            int y = j >= 0 ? b[j] - '0' : 0;
            int sum = x + y + carry; 
            ans += sum % 2 + '0';
            carry = sum / 2;
            i--;
            j--;
        }
        ranges::reverse(ans);
        return ans;
    }
};