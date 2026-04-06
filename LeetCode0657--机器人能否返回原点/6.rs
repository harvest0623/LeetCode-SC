impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        
        for ch in moves.chars() {
            match ch {
                'U' => y -= 1,
                'D' => y += 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => continue,
            }
        }
        
        x == 0 && y == 0
    }
}