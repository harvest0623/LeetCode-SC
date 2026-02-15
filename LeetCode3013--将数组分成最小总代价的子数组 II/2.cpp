class Solution {
public:
    long long minimumCost(vector<int>& nums, int k, int dist) {
        k--;
        long long sum = reduce(nums.begin(), nums.begin() + dist + 2, 0LL);
        multiset<int> L(nums.begin() + 1, nums.begin() + dist + 2), R;
        auto L2R = [&]() {
            int x = *L.rbegin();
            sum -= x;
            L.erase(L.find(x));
            R.insert(x);
        };
        auto R2L = [&]() {
            int x = *R.begin();
            sum += x;
            R.erase(R.find(x));
            L.insert(x);
        };
        while (L.size() > k) {
            L2R();
        }

        long long ans = sum;
        for (int i = dist + 2; i < nums.size(); i++) {
            // 移除 out
            int out = nums[i - dist - 1];
            auto it = L.find(out);
            if (it != L.end()) {
                sum -= out;
                L.erase(it);
            } else {
                R.erase(R.find(out));
            }

            // 添加 in
            int in = nums[i];
            if (in < *L.rbegin()) {
                sum += in;
                L.insert(in);
            } else {
                R.insert(in);
            }

            // 维护大小
            if (L.size() == k - 1) {
                R2L();
            } else if (L.size() == k + 1) {
                L2R();
            }

            ans = min(ans, sum);
        }
        return ans;
    }
};