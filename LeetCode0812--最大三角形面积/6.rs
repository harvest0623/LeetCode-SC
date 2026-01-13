impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut max_area = 0.0;
        
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let area = Self::triangle_area(
                        points[i][0], points[i][1],
                        points[j][0], points[j][1],
                        points[k][0], points[k][1]
                    );
                    max_area = max_area.max(area);
                }
            }
        }
        
        max_area
    }
    
    fn triangle_area(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> f64 {
        0.5 * ((x1 * y2 + x2 * y3 + x3 * y1 - x1 * y3 - x2 * y1 - x3 * y2) as f64).abs()
    }
}