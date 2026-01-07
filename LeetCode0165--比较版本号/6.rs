impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let a = version1.split('.').collect::<Vec<_>>();
        let b = version2.split('.').collect::<Vec<_>>();
        let n = a.len();
        let m = b.len();
        for i in 0..n.max(m) {
            let ver1 = if i < n { a[i].parse::<i32>().unwrap() } else { 0 };
            let ver2 = if i < m { b[i].parse::<i32>().unwrap() } else { 0 };
            if ver1 != ver2 {
                return if ver1 < ver2 { -1 } else { 1 };
            }
        }
        0
    }
}