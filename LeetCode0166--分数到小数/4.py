class Solution:
    def fractionToDecimal(self, numerator: int, denominator: int) -> str:
        sign = '-' if numerator * denominator < 0 else ''
        numerator = abs(numerator) 
        denominator = abs(denominator)
        q, r = divmod(numerator, denominator)
        if r == 0: 
            return sign + str(q)
        ans = [sign + str(q) + '.']
        r_to_pos = {r: 1} 
        while r:
            q, r = divmod(r * 10, denominator)
            ans.append(str(q))
            if r in r_to_pos:  
                pos = r_to_pos[r]  
                return f"{''.join(ans[:pos])}({''.join(ans[pos:])})"
            r_to_pos[r] = len(ans)  
        return ''.join(ans) 