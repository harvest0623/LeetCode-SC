// 长除法
var fractionToDecimal = function(numerator, denominator) {
    const sign = numerator * denominator < 0 ? "-" : "";
    numerator = Math.abs(numerator); 
    denominator = Math.abs(denominator);
    let q = Math.floor(numerator / denominator);
    let r = numerator % denominator;
    if (r === 0) { 
        return sign + String(q);
    }
    const ans = [sign + String(q) + "."];
    const r_to_pos = new Map();
    r_to_pos.set(r, 1); 
    while (r) {
        r *= 10;
        q = Math.floor(r / denominator);
        r = r % denominator;
        ans.push(String(q));
        if (r_to_pos.has(r)) { 
            const pos = r_to_pos.get(r); 
            return ans.slice(0, pos).join("") + "(" + ans.slice(pos).join("") + ")";
        }
        r_to_pos.set(r, ans.length); 
    }
    return ans.join(""); 
};