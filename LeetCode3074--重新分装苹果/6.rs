impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut s = apple.iter().sum::<i32>(); 
        capacity.sort_unstable_by_key(|a| -a);
        for (i, x) in capacity.iter().enumerate() {
            s -= x;
            if s <= 0 { 
                return (i + 1) as _; 
            }
        }
        unreachable!()
    }
}