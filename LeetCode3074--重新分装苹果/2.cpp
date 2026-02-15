class Solution {
public:
    int minimumBoxes(vector<int>& apple, vector<int>& capacity) {
        int s = reduce(apple.begin(), apple.end()); 
        ranges::sort(capacity, greater()); 
        int i = 0;
        while (s > 0) { 
            s -= capacity[i++]; 
        }
        return i;
    }
};