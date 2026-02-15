var addBinary = function(a, b) {
    const ans = [];
    let i = a.length - 1; // 从右往左遍历 a 和 b
    let j = b.length - 1;
    let carry = 0; // 保存进位
    while (i >= 0 || j >= 0 || carry) {
        const x = i >= 0 ? Number(a[i]) : 0;
        const y = j >= 0 ? Number(b[j]) : 0;
        const sum = x + y + carry; // 计算这一位的加法
        // 例如 sum = 10，把 '0' 填入答案，把 carry 置为 1
        ans.push(String(sum % 2));
        carry = Math.floor(sum / 2);
        i--;
        j--;
    }
    return ans.reverse().join('');
};