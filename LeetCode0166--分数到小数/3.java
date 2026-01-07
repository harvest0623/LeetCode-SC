class Solution {
    public String fractionToDecimal(int numerator, int denominator) {
        long a = numerator;
        long b = denominator;
        String sign = a * b < 0 ? "-" : "";
        a = Math.abs(a); 
        b = Math.abs(b);
        long q = a / b;
        long r = a % b;
        if (r == 0) { 
            return sign + q;
        }
        StringBuilder ans = new StringBuilder(sign).append(q).append('.');
        Map<Long, Integer> rToPos = new HashMap<>();
        rToPos.put(r, ans.length()); 
        while (r > 0) {
            r *= 10;
            q = r / b;
            r %= b;
            ans.append(q);
            if (rToPos.containsKey(r)) { 
                int pos = rToPos.get(r); 
                return ans.substring(0, pos) + "(" + ans.substring(pos) + ")";
            }
            rToPos.put(r, ans.length()); 
        }
        return ans.toString(); 
    }
}