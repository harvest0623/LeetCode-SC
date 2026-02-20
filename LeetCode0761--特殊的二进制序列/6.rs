impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut cnt = 0;
        let mut arr = s.split_inclusive(|ch| {
            cnt += if ch == '1' { 1 } else { -1 };
            cnt == 0
        }).map(|curr| format!("1{}0", Self::make_largest_special(curr[1..curr.len() - 1].to_string()))).collect::<Vec<_>>();
        arr.sort_by(|a, b| b.cmp(a));
        arr.concat()
    }
}