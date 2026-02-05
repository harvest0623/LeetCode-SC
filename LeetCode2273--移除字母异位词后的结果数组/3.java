class Solution {
    public List<String> removeAnagrams(String[] words) {
        char[] base = words[0].toCharArray();
        Arrays.sort(base);
        int k = 1;
        for (int i = 1; i < words.length; i++) {
            char[] s = words[i].toCharArray();
            Arrays.sort(s);
            if (!Arrays.equals(s, base)) {
                base = s;
                words[k++] = words[i]; // 保留 words[i]
            }
        }
        return Arrays.asList(Arrays.copyOf(words, k));
    }
}