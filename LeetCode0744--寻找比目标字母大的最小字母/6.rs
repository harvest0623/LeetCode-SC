impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &letter in &letters {
            if letter > target {
                return letter;
            }
        }
        letters[0]
    }
}