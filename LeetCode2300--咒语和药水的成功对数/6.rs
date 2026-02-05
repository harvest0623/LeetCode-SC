impl Solution {
    pub fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        let last = potions[potions.len() - 1] as i64;
        for x in spells.iter_mut() {
            let target = success as f64 / *x as f64;
            let j = potions.partition_point(|&x| (x as f64) < target);
            *x = (potions.len() - j) as i32;
        }
        spells
    }
}