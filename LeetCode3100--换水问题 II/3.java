// 方法一：模拟
class Solution {
    public int maxBottlesDrunk(int numBottles, int numExchange) {
        int ans = numBottles;
        for (int empty = numBottles; empty >= numExchange; numExchange++) {
            ans++;
            empty -= numExchange - 1;
        }
        return ans;
    }
}

// 方法二：数学
class Solution {
    public int maxBottlesDrunk(int numBottles, int numExchange) {
        int t = 0;
        int empty = t * numExchange + t * (t - 1) / 2;
        int total = numBottles + t;
        int a = 1;
        int b = 2 * numExchange - 3;
        int c = -2 * numBottles;
        t = (int) Math.ceil(((-b + Math.sqrt(b * b - 4 * a * c)) / (2 * a)));
        return numBottles + t - 1;
    }
}