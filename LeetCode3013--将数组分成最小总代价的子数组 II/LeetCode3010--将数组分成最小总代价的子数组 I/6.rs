use std::cmp::min;
use std::collections::BTreeSet;

fn l_2_r(ltree: &mut BTreeSet<(i32, usize)>, rtree: &mut BTreeSet<(i32, usize)>, mut sum_left: i64) -> i64 {
    let x: (i32, usize) = ltree.pop_last().unwrap();
    sum_left = sum_left - x.0 as i64;
    rtree.insert(x);
    return sum_left;
}

fn r_2_l(ltree: &mut BTreeSet<(i32, usize)>, rtree: &mut BTreeSet<(i32, usize)>, mut sum_left: i64) -> i64 {
    let x: (i32, usize) = rtree.pop_first().unwrap();
    sum_left = sum_left + x.0 as i64;
    ltree.insert(x);
    return sum_left;
}
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let kk: i32 = k - 1;
        let mut ltree = BTreeSet::new();
        let mut rtree = BTreeSet::new();
        let mut sum_left: i64 = 0;
        for (id, v) in nums.iter().enumerate() {
            if id == (dist + 2) as usize {
                break;
            }
            sum_left = sum_left + *v as i64;
            if id != 0 {
                ltree.insert((*v, id));
            }
        }
        while ltree.len() > kk as usize {
            sum_left = l_2_r(&mut ltree, &mut rtree, sum_left);
        }
        let mut ans: i64 = sum_left;
        for (id, v) in nums.iter().enumerate() {
            if id >= (dist + 2) as usize {
                let out: (i32, usize) = (nums[(id as i32- dist - 1) as usize] ,(id as i32- dist - 1) as usize);
                if ltree.contains(&out) {
                    sum_left = sum_left - out.0 as i64;
                    ltree.remove(&out);
                } else {
                    rtree.remove(&out);
                }
                let in_val: (i32, usize) = (*v, id);
                if *v < ltree.last().unwrap().0  {
                    sum_left = sum_left + *v as i64;
                    ltree.insert(in_val);
                } else {
                    rtree.insert(in_val);
                }
                if ltree.len() == (kk - 1) as usize {
                    sum_left = r_2_l(&mut ltree, &mut rtree, sum_left);
                } else if ltree.len() == (kk + 1) as usize {
                    sum_left = l_2_r(&mut ltree, &mut rtree, sum_left);
                }
                ans = min(ans, sum_left);
            }
        }
        return ans;
    }
}