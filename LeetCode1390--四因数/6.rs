impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for &num in &nums {
            let mut cnt = 0;
            let mut sum = 0;
            let mut i = 1;
            while i * i <= num {
                if num % i == 0 {
                    cnt += 1;
                    sum += i;
                    if i * i != num {   
                        cnt += 1;
                        sum += num / i;
                    }
                }
                i += 1;
            }
            if cnt == 4 {
                ans += sum;
            }
        }
        ans
    }
}