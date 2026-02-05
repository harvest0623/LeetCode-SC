class Solution {
public:
    vector<int> successfulPairs(vector<int>& spells, vector<int>& potions, long long success) {
        ranges::sort(potions);
        for (int& x : spells) { 
            x = potions.end() - ranges::lower_bound(potions, 1.0 * success / x);
        }
        return spells;
    }
};