class Solution {
public:
    vector<string> removeAnagrams(vector<string>& words) {
        string base = "";
        int k = 0;
        for (auto& word : words) {
            string s = word;
            ranges::sort(s);
            if (s != base) {
                base = move(s);
                words[k++] = word; // 保留 word
            }
        }
        words.resize(k);
        return words;
    }
};