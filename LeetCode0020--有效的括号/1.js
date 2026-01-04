var isValid = function(s) {
    const obj = {
        '()' : 1,
        '[]' : 1,
        '{}' : 1
    };
    const stack = [];
    const len = s.length;
    for(let i = 0; i < len; i++){
        const ch = s[i];
        if (ch == '(' || ch == '[' || ch == '{'){
            stack.push(ch);
        } else{  
            const top = stack.pop();
            const match = top + ch;
            if(obj[match] !== 1){
                return false;
            }
        }
    }
    return stack.length == 0;
};