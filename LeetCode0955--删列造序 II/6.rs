impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let mut cuts = vec![false; a.len() - 1];
        let mut ans = 0;
        for j in 0..a[0].len() {
            let mut can_keep = true;
            for i in 0..a.len() - 1 {
                if !cuts[i] {
                    let char_i = a[i].chars().nth(j).unwrap();
                    let char_i1 = a[i+1].chars().nth(j).unwrap();
                    if char_i > char_i1 {
                        can_keep = false;
                        break;
                    }
                }
            }
            if can_keep {
                for i in 0..a.len() - 1 {
                    let char_i = a[i].chars().nth(j).unwrap();
                    let char_i1 = a[i+1].chars().nth(j).unwrap();
                    if char_i < char_i1 {
                        cuts[i] = true;
                    }
                }
            } else {
                ans += 1;
            }
        }
        ans
    }
}