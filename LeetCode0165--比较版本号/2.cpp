// 方法一：字符串分割
class Solution {
public:
    vector<int> splitVersion(const string& version) {
        vector<int> res;
        stringstream ss(version);
        string part;
        // 按 '.' 分割字符串
        while (getline(ss, part, '.')) {
            // 转换为整数（自动忽略前导零）
            res.push_back(stoi(part));
        }
        return res;
    }
    int compareVersion(string version1, string version2) {
        vector<int> v1 = splitVersion(version1);
        vector<int> v2 = splitVersion(version2);
        for (int i = 0; i < v1.size() || i < v2.size(); ++i) {
            int x = 0, y = 0;
            if (i < v1.size()) {
                x = v1[i];
            }
            if (i < v2.size()) {
                y = v2[i];
            }
            if (x > y) {
                return 1;
            }
            if (x < y) {
                return -1;
            }
        }
        return 0;
    }
};

// 方法二：双指针
class Solution {
public:
    int compareVersion(string version1, string version2) {
        int n = version1.length(), m = version2.length();
        int i = 0, j = 0;
        while (i < n || j < m) {
            long long x = 0;
            for (; i < n && version1[i] != '.'; ++i) {
                x = x * 10 + version1[i] - '0';
            }
            ++i; // 跳过点号
            long long y = 0;
            for (; j < m && version2[j] != '.'; ++j) {
                y = y * 10 + version2[j] - '0';
            }
            ++j; // 跳过点号
            if (x != y) {
                return x > y ? 1 : -1;
            }
        }
        return 0;
    }
};