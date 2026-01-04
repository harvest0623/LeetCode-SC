use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 { // s 长度必须是偶数
            return false;
        }
        let mp = [(b')', b'('), (b']', b'['), (b'}', b'{')].iter().cloned().collect::<HashMap<_, _>>();
        let mut st = vec![];
        for c in s.bytes() {
            if !mp.contains_key(&c) { // c 是左括号
                st.push(c); // 入栈
            } else if st.is_empty() || st.pop().unwrap() != *mp.get(&c).unwrap() { // c 是右括号
                return false; // 没有左括号，或者左括号类型不对
            }
        }
        st.is_empty() // 所有左括号必须匹配完毕
    }
}