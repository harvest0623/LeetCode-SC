class Solution {
    public String makeLargestSpecial(String s) {
        if (s.length() <= 2) return s;
        int cnt = 0, left = 0;
        List<String> subs = new ArrayList<String>();
        for (int i = 0; i < s.length(); ++i) {
            if (s.charAt(i) == '1') {
                ++cnt;
            } else {
                --cnt;
                if (cnt == 0) {
                    subs.add("1" + makeLargestSpecial(s.substring(left + 1, i)) + "0");
                    left = i + 1;
                }
            }
        }
        Collections.sort(subs, (a, b) -> b.compareTo(a));
        StringBuilder ans = new StringBuilder();
        for (String sub : subs) {
            ans.append(sub);
        }
        return ans.toString();
    }
}