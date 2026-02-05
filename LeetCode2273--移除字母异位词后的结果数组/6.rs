impl Solution {
    pub fn remove_anagrams(mut words: Vec<String>) -> Vec<String> {
        let mut base = vec![];
        let mut k = 0;
        for i in 0..words.len() {
            let mut s = words[i].as_bytes().to_vec();
            s.sort_unstable();
            if s != base {
                base = s;
                words[k] = words[i].clone(); // 保留 words[i]
                k += 1;
            }
        }
        words.truncate(k);
        words
    }
}