class Solution {
    public String longestCommonPrefix(String[] strs) {
        String s0 = strs[0];
        for (int j = 0; j < s0.length(); j++) { 
            char c = s0.charAt(j);
            for (String s : strs) { 
                if (j == s.length() || s.charAt(j) != c) { 
                    return s0.substring(0, j); 
                }
            }
        }
        return s0;
    }
}