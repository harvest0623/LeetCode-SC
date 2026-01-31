impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let n = prices.len();
        let mut res: i64 = 1;   
        let mut prev: i32 = 1;  
        for i in 1..n {
            if prices[i] == prices[i - 1] - 1 {
                prev += 1;    
            } else {
                prev = 1;
            }
            res += prev as i64;
        }
        res
    }
}