class Solution {
public:
    vector<int> sortByBits(vector<int>& arr) {
        ranges::sort(arr, {}, [](int x) {
            return pair(popcount((uint32_t) x), x);
        });
        return arr;
    }
};